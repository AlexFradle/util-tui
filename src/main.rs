#[macro_use]
extern crate lazy_static;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use db::DB;
use log::info;
use screens::Screen;
use sqlx::Connection;
use std::{error::Error, future::Future, io, pin::Pin};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use util::{set_backlight, set_volume};
use wait_popup::WaitPopup;

mod app;
mod button;
mod calendar;
mod clock;
mod db;
mod film_tracker;
mod form;
mod grade_tracker;
mod money_tracker;
mod popup;
mod progress_bar;
mod screens;
mod styles;
mod util;
mod wait_popup;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup logging
    // https://tms-dev-blog.com/log-to-a-file-in-rust-with-log4rs/
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    DB::create_tables().await;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new().await;
    let res = run_app(&mut terminal, app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let c = terminal.draw(|f| {
            app.cur_screen.get_screen_func()(f, &mut app);
        })?;

        let mut futs = Vec::new();
        if let Event::Key(key) = event::read()? {
            // quit if read_input returns true
            if read_input(&mut app, &key, &mut futs) {
                break;
            }
        }
        let ob = c.buffer.clone();
        if !futs.is_empty() {
            terminal.draw(|f| {
                let wait_popup = WaitPopup::new(&ob);
                f.render_widget(wait_popup, f.size());
            })?;

            // execute all futures from the read_input func
            for f in futs {
                f.await;
            }
        }
    }
    app.db.conn.close();
    return Ok(());
}

fn read_input<'a>(
    app: &'a mut App,
    key: &KeyEvent,
    futs: &mut Vec<Pin<Box<dyn Future<Output = ()> + 'a>>>,
) -> bool {
    // https://users.rust-lang.org/t/storing-futures/34564/8

    let capture_input = app.grade_state.show_form
        || app.money_state.add_form_selected
        || app.money_state.search_form_selected;

    match (&app.cur_screen, key.code, capture_input) {
        // Dashboard Screen ---------------------------------------------------
        (Screen::DashboardScreen, KeyCode::Left, _) => {
            app.brightness -= 1;
            set_backlight(app.brightness);
        }
        (Screen::DashboardScreen, KeyCode::Right, _) => {
            app.brightness += 1;
            set_backlight(app.brightness);
        }
        (Screen::DashboardScreen, KeyCode::Up, _) => {
            app.volume += 1;
            set_volume(app.volume);
        }
        (Screen::DashboardScreen, KeyCode::Down, _) => {
            app.volume -= 1;
            set_volume(app.volume);
        }

        // Calendar Screen ----------------------------------------------------
        (Screen::CalendarScreen, KeyCode::Down, _) => {
            futs.push(Box::pin(app.calendar_state.increment_month(-1)));
        }
        (Screen::CalendarScreen, KeyCode::Up, _) => {
            futs.push(Box::pin(app.calendar_state.increment_month(1)));
        }
        (Screen::CalendarScreen, KeyCode::Left, _) => {
            if app.calendar_state.show_popup {
                app.calendar_state.increment_selected_event(-1)
            } else {
                app.calendar_state.increment_selected_day(-1)
            }
        }
        (Screen::CalendarScreen, KeyCode::Right, _) => {
            if app.calendar_state.show_popup {
                app.calendar_state.increment_selected_event(1)
            } else {
                app.calendar_state.increment_selected_day(1)
            }
        }
        (Screen::CalendarScreen, KeyCode::Enter, _) => app.calendar_state.popup_toggle(),

        // Grade Screen -------------------------------------------------------
        (Screen::GradeScreen, KeyCode::Up, false) => app.grade_state.increment_selected(-1),
        (Screen::GradeScreen, KeyCode::Up, true) => {
            app.grade_state.form_state.increment_selected(-1);
        }
        (Screen::GradeScreen, KeyCode::Down, false) => app.grade_state.increment_selected(1),
        (Screen::GradeScreen, KeyCode::Down, true) => {
            app.grade_state.form_state.increment_selected(1);
        }
        (Screen::GradeScreen, KeyCode::Tab, true) => {
            app.grade_state.form_state.increment_selected(1);
        }
        (Screen::GradeScreen, KeyCode::Esc, true) => {
            app.grade_state.toggle_form();
        }
        (Screen::GradeScreen, KeyCode::Char('i'), false) => {
            app.grade_state.toggle_form();
        }
        (Screen::GradeScreen, KeyCode::Char(_) | KeyCode::Backspace, true) => {
            app.grade_state.form_state.send_input(&key.code);
        }
        (Screen::GradeScreen, KeyCode::Enter, true) => {
            app.grade_state.submit_form();
            app.grade_state.toggle_form();
        }

        // Money Screen -------------------------------------------------------
        (Screen::MoneyScreen, KeyCode::Char('i'), false) => {
            app.money_state.select_add_form();
        }
        (Screen::MoneyScreen, KeyCode::Char('s'), false) => {
            app.money_state.select_search_form();
        }
        (Screen::MoneyScreen, KeyCode::Esc, true) => {
            app.money_state.select_transaction_list();
        }
        (Screen::MoneyScreen, KeyCode::Char(_) | KeyCode::Backspace, true) => {
            if app.money_state.search_form_selected {
                app.money_state.search_form.send_input(&key.code);
            } else {
                app.money_state.add_form.send_input(&key.code);
            }
        }
        (Screen::MoneyScreen, KeyCode::Up, false) => app.money_state.increment_selected(-1),
        (Screen::MoneyScreen, KeyCode::Up, true) => {
            if app.money_state.search_form_selected {
                app.money_state.search_form.increment_selected(-1);
            } else {
                app.money_state.add_form.increment_selected(-1);
            }
        }
        (Screen::MoneyScreen, KeyCode::Down, false) => app.money_state.increment_selected(1),
        (Screen::MoneyScreen, KeyCode::Down, true) => {
            if app.money_state.search_form_selected {
                app.money_state.search_form.increment_selected(1);
            } else {
                app.money_state.add_form.increment_selected(1);
            }
        }
        (Screen::MoneyScreen, KeyCode::Tab, true) => {
            if app.money_state.search_form_selected {
                app.money_state.search_form.increment_selected(1);
            } else {
                app.money_state.add_form.increment_selected(1);
            }
        }
        (Screen::MoneyScreen, KeyCode::Enter, true) => {
            if app.money_state.search_form_selected {
                futs.push(Box::pin(app.money_state.submit_search_form(&mut app.db)));
            } else {
                futs.push(Box::pin(app.money_state.submit_add_form(&mut app.db)));
            }
        }
        (Screen::MoneyScreen, KeyCode::Right, false) => {
            futs.push(Box::pin(app.money_state.get_next_page(&mut app.db)));
        }
        (Screen::MoneyScreen, KeyCode::Left, false) => {
            futs.push(Box::pin(app.money_state.get_prev_page(&mut app.db)));
        }

        // Film Screen --------------------------------------------------------
        (Screen::FilmScreen, KeyCode::Enter, false) => {
            futs.push(Box::pin(app.film_state.search_movie("dark".to_owned())));
        }

        // All Screens --------------------------------------------------------
        (_, KeyCode::Char('d'), false) => app.cur_screen = Screen::DashboardScreen,
        (_, KeyCode::Char('c'), false) => app.cur_screen = Screen::CalendarScreen,
        (_, KeyCode::Char('g'), false) => app.cur_screen = Screen::GradeScreen,
        (_, KeyCode::Char('m'), false) => app.cur_screen = Screen::MoneyScreen,
        (_, KeyCode::Char('f'), false) => app.cur_screen = Screen::FilmScreen,
        (_, KeyCode::Char('q'), false) => return true,

        _ => {}
    };
    return false;
}
