use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::App,
    calendar::{Calendar, CalendarEvent},
    clock::Clock,
    grade_tracker::GradeTracker,
    money_tracker::MoneyTracker,
    popup::Popup,
    progress_bar::ProgressBar,
    styles::AppStyles,
};

pub enum Screen {
    DashboardScreen,
    CalendarScreen,
    GradeScreen,
    MoneyScreen,
}

impl Screen {
    pub fn get_screen_func<B: Backend>(&self) -> impl Fn(&mut Frame<B>, &mut App) {
        match self {
            Screen::DashboardScreen => dashboard_screen,
            Screen::CalendarScreen => calendar_screen,
            Screen::GradeScreen => grade_screen,
            Screen::MoneyScreen => money_screen,
        }
    }
}

fn dashboard_screen<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let clock = Clock::new(true);

    let area = f.size();

    let clock_rect = Rect {
        x: 0,
        y: 0,
        width: area.width / 2,
        height: area.height / 2,
    };

    let bb_rect = Rect {
        x: 0,
        y: area.height - 6,
        width: area.width / 2,
        height: 3,
    };

    let vb_rect = Rect {
        x: 0,
        y: area.height - 3,
        width: area.width / 2,
        height: 3,
    };

    let block_rect = Rect {
        x: 0,
        y: area.height / 2,
        width: area.width / 2,
        height: area.height / 2 - 6,
    };

    f.render_stateful_widget(clock, clock_rect, &mut app.clock_state);

    let brightness_bar = ProgressBar::new("Brightness".to_owned(), app.brightness);
    f.render_widget(brightness_bar.get_gauge(), bb_rect);

    let volume_bar = ProgressBar::new("Volume".to_owned(), app.volume);
    f.render_widget(volume_bar.get_gauge(), vb_rect);

    let empty_block = Block::default()
        .borders(Borders::ALL)
        .style(AppStyles::Main.get());
    f.render_widget(empty_block, block_rect);

    let block_rect = Rect {
        x: area.width / 2,
        y: area.height / 3,
        width: area.width / 2,
        height: area.height - area.height / 3,
    };

    let empty_block = Block::default()
        .borders(Borders::ALL)
        .title("TODO")
        .style(AppStyles::Main.get());
    f.render_widget(empty_block, block_rect);

    let block_rect = Rect {
        x: area.width / 2,
        y: 0,
        width: area.width / 2,
        height: area.height / 3,
    };

    let empty_block = Block::default()
        .borders(Borders::ALL)
        .title("Calendar")
        .style(AppStyles::Main.get());
    f.render_widget(empty_block, block_rect);
}

fn calendar_screen<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let cal = Calendar::new();
    f.render_stateful_widget(cal, f.size(), &mut app.calendar_state);
    if app.calendar_state.show_popup {
        let empty_vec: Vec<CalendarEvent> = Vec::new();
        let cur_data = app
            .calendar_state
            .data
            .get(&app.calendar_state.selected_day)
            .unwrap_or(&empty_vec);

        let all_events_text: Vec<Vec<Spans>> = cur_data
            .iter()
            .enumerate()
            .map(|(i, event)| {
                let mut v = vec![Spans::from(Span::raw(&event.title))];
                v.append(
                    &mut event
                        .description
                        .split("\n")
                        .map(|s| Spans::from(Span::raw(s)))
                        .collect::<Vec<Spans>>(),
                );

                v.push(Spans::from(Span::raw(format!(
                    " {}/{} ",
                    i + 1,
                    cur_data.len()
                ))));
                return v;
            })
            .collect();

        if all_events_text.len() > 0 {
            let lines = all_events_text[app.calendar_state.selected_event as usize].clone();
            let p = Paragraph::new(lines)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            let popup = Popup::new(p, 50, 50);
            f.render_widget(popup, f.size());
        } else {
            app.calendar_state.popup_toggle();
        }
    }
}

fn grade_screen<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let g = GradeTracker::new();
    f.render_stateful_widget(g, f.size(), &mut app.grade_state);
}

fn money_screen<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let m = MoneyTracker::new();
    f.render_stateful_widget(m, f.size(), &mut app.money_state);
}
