use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use screens::Screen;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use util::{set_backlight, set_volume};

mod app;
mod calendar;
mod clock;
mod json_date_format;
mod list;
mod progress_bar;
mod screens;
mod styles;
mod table;
mod util;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| app.cur_screen.get_screen_func()(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match (&app.cur_screen, key.code) {
                (_, KeyCode::Char('q')) => return Ok(()),
                (Screen::DashboardScreen, KeyCode::Left) => {
                    app.brightness -= 1;
                    set_backlight(app.brightness);
                }
                (Screen::DashboardScreen, KeyCode::Right) => {
                    app.brightness += 1;
                    set_backlight(app.brightness);
                }
                (Screen::DashboardScreen, KeyCode::Up) => {
                    app.volume += 1;
                    set_volume(app.volume);
                }
                (Screen::DashboardScreen, KeyCode::Down) => {
                    app.volume -= 1;
                    set_volume(app.volume);
                }
                (Screen::CalendarScreen, KeyCode::Down) => app.calendar_state.increment_month(-1),
                (Screen::CalendarScreen, KeyCode::Up) => app.calendar_state.increment_month(1),
                (Screen::CalendarScreen, KeyCode::Left) => {
                    app.calendar_state.increment_selected(-1)
                }
                (Screen::CalendarScreen, KeyCode::Right) => {
                    app.calendar_state.increment_selected(1)
                }
                (_, KeyCode::Char('c')) => app.cur_screen = Screen::CalendarScreen,
                _ => {}
            }
        }
    }
}
