use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod calendar;
mod list;
mod progress_bar;
mod styles;
mod table;
mod ui;

use crate::app::App;
use crate::ui::ui;

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
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.test_table.next(),
                KeyCode::Up => app.test_table.previous(),
                KeyCode::Char('n') => app.test_list.next(),
                KeyCode::Char('a') => match app.test_table.add_row(&["a", "b", "c"]) {
                    Ok(_) => {}
                    Err(e) => println!("{e}"),
                },
                KeyCode::Char('x') => app.progress += 1,
                KeyCode::Char('l') => app.calendar.state.increment_month(1),
                KeyCode::Char('p') => app.calendar.state.increment_month(-1),
                KeyCode::Left => app.calendar.state.increment_selected(-1),
                KeyCode::Right => app.calendar.state.increment_selected(1),
                _ => {}
            }
        }
    }
}
