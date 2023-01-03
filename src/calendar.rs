use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, Utc};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

pub struct Calendar<'a> {
    pub data: Vec<(u8, &'a str)>,
    pub selected: u8,
    date_string: String,
    num_of_days: i64,
    start_day: u32,
}

impl<'a> Calendar<'a> {
    pub fn new() -> Calendar<'a> {
        let local_time: DateTime<Local> = Local::now();
        let cur_day = local_time.day();
        let cur_month = local_time.month();
        let cur_year = local_time.year();
        // Mon 01 Jan 2022
        let date_string = format!("{}", local_time.format("%a %d %b %Y"));
        let num_of_days = NaiveDate::from_ymd(
            match cur_month {
                12 => cur_year + 1,
                _ => cur_year,
            },
            match cur_month {
                12 => 1,
                _ => cur_month + 1,
            },
            1,
        )
        .signed_duration_since(NaiveDate::from_ymd(cur_year, cur_month, 1))
        .num_days();
        // 0 = Monday, 6 = Sunday
        let start_day = NaiveDate::from_ymd(cur_year, cur_month, 1)
            .weekday()
            .number_from_monday()
            - 1;

        Calendar {
            data: vec![],
            selected: 0,
            date_string,
            num_of_days,
            start_day,
        }
    }

    pub fn change_selected(&mut self, new_index: u8) {
        self.selected = new_index;
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let main_layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .split(area);
        let main_block = Block::default().title("Calendar").borders(Borders::ALL);

        let layout = Layout::default()
            .constraints(
                [
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .horizontal_margin(4)
            .split(main_layout[0]);

        f.render_widget(main_block, main_layout[0]);

        for i in 0..6 {
            let line_layout = Layout::default()
                .constraints(
                    [
                        Constraint::Ratio(1, 7),
                        Constraint::Ratio(1, 7),
                        Constraint::Ratio(1, 7),
                        Constraint::Ratio(1, 7),
                        Constraint::Ratio(1, 7),
                        Constraint::Ratio(1, 7),
                        Constraint::Ratio(1, 7),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Horizontal)
                .split(layout[i + 1]);
            for j in 0..7 {
                let mut day: i32 = j + (7 * i) as i32 - self.start_day as i32;
                if day < 1 {
                    day = 0;
                }
                let b = Block::default()
                    .title(format!("{}", day))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL);
                f.render_widget(b, line_layout[j as usize]);
            }
        }
    }
}
