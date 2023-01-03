use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Gauge, Row, Table, TableState},
    Frame, Terminal,
};

use crate::{app::App, calendar::Calendar, progress_bar::ProgressBar, styles::AppStyles};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_layout = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());
    let c = Calendar::new();
    c.render(f, main_layout[0]);
    //     let rects = Layout::default()
    //         .constraints(
    //             [
    //                 Constraint::Percentage(50),
    //                 Constraint::Percentage(20),
    //                 Constraint::Percentage(10),
    //             ]
    //             .as_ref(),
    //         )
    //         .direction(Direction::Vertical)
    //         .margin(1)
    //         .split(f.size());
    //
    //     let prog = ProgressBar::new(app.progress);
    //
    //     f.render_stateful_widget(
    //         app.test_table.get_table(),
    //         rects[0],
    //         &mut app.test_table.state,
    //     );
    //
    //     f.render_stateful_widget(app.test_list.get_list(), rects[1], &mut app.test_list.state);
    //
    //     let block_rects = Layout::default()
    //         .constraints([
    //             Constraint::Percentage(25),
    //             Constraint::Percentage(25),
    //             Constraint::Percentage(25),
    //             Constraint::Percentage(25),
    //         ])
    //         .direction(Direction::Horizontal)
    //         .split(rects[2]);
    //
    //     let block = Block::default()
    //         .title("Block1")
    //         .borders(Borders::ALL)
    //         .border_type(BorderType::Plain)
    //         .title_alignment(Alignment::Center)
    //         .style(AppStyles::Main.get());
    //
    //     f.render_widget(block, block_rects[0]);
    //
    //     let block = Block::default()
    //         .title("Block2")
    //         .borders(Borders::ALL)
    //         .border_type(BorderType::Rounded)
    //         .style(AppStyles::Main.get());
    //
    //     f.render_widget(block, block_rects[1]);
    //
    //     let block = Block::default()
    //         .title("Block3")
    //         .borders(Borders::ALL)
    //         .border_type(BorderType::Double)
    //         .style(AppStyles::Main.get());
    //
    //     f.render_widget(block, block_rects[2]);
    //
    //     let block = Block::default()
    //         .title("Block4")
    //         .borders(Borders::ALL)
    //         .border_type(BorderType::Thick)
    //         .style(AppStyles::Main.get());
    //
    //     f.render_widget(block, block_rects[3]);
}
