use chrono::{DateTime, Datelike, Local, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::styles::AppStyles;

pub struct CalendarState {
    pub selected: u32,
    pub cur_day: u32,
    pub cur_month: Month,
    pub cur_year: i32,
    pub num_of_days: i64,
    pub start_day: u32,
}

impl CalendarState {
    pub fn new() -> CalendarState {
        let local_time: DateTime<Local> = Local::now();
        let cur_day = local_time.day();
        let cur_month = local_time.month();
        let cur_year = local_time.year();
        // Mon 01 Jan 2022
        // let date_string = format!("{}", local_time.format("%a %d %b %Y"));
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

        CalendarState {
            selected: cur_day,
            cur_day,
            cur_month: Month::from_u32(cur_month).unwrap(),
            cur_year,
            num_of_days,
            start_day,
        }
    }

    fn set_num_of_days(&mut self) {
        let cm = self.cur_month.number_from_month();
        self.num_of_days = NaiveDate::from_ymd(
            match cm {
                12 => self.cur_year + 1,
                _ => self.cur_year,
            },
            match cm {
                12 => 1,
                _ => cm + 1,
            },
            1,
        )
        .signed_duration_since(NaiveDate::from_ymd(self.cur_year, cm, 1))
        .num_days();
    }

    fn set_start_day(&mut self) {
        self.start_day = NaiveDate::from_ymd(self.cur_year, self.cur_month.number_from_month(), 1)
            .weekday()
            .number_from_monday()
            - 1;
    }

    pub fn increment_month(&mut self, amount: i32) {
        let mut new_month: Month = self.cur_month;
        for _ in 0..amount.abs() {
            if amount.is_positive() {
                new_month = new_month.succ();
                if new_month == Month::January {
                    self.cur_year += 1;
                }
            } else {
                new_month = new_month.pred();
                if new_month == Month::December {
                    self.cur_year -= 1;
                }
            }
        }
        self.cur_month = new_month;
        self.set_start_day();
        self.set_num_of_days();
    }

    pub fn increment_selected(&mut self, amount: i32) {
        if self.selected > 1 && amount.is_negative() {
            self.selected -= amount.abs() as u32;
        } else if self.selected < self.num_of_days as u32 && amount.is_positive() {
            self.selected += amount.abs() as u32;
        }
    }
}

pub struct CalendarObj<'a> {
    pub state: CalendarState,
    pub pointless: &'a str,
}

impl<'a> CalendarObj<'a> {
    pub fn new() -> CalendarObj<'a> {
        CalendarObj {
            state: CalendarState::new(),
            pointless: "dsa",
        }
    }

    pub fn get_calendar(&self) -> Calendar<'a> {
        Calendar::new()
    }

    pub fn next_month(&mut self) {
        self.state.increment_month(1);
    }
}

pub struct Calendar<'a> {
    pub data: Vec<(u8, &'a str)>,
}

impl<'a> StatefulWidget for Calendar<'a> {
    type State = CalendarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // + 1 to avoid border
        let cx = area.left() + 1;
        // + 1 to avoid border
        let cy = area.top() + 1;
        // - 2 for top and bottom border
        let width = area.width - 2;
        // - 2 for top and bottom border, - 1 for day line
        let height = area.height - 2 - 1;
        let cell_width = width / 7;
        let cell_height = height / 6;

        let month_str = format!("{} {}", state.cur_month.name(), state.cur_year);
        buf.set_string(
            cx + width / 2 - (month_str.len() / 2) as u16,
            cy,
            month_str,
            Style::default(),
        );

        let borders = Borders::ALL;

        let offset_x = (width - cell_width * 7) / 2;
        let offset_y = (height - cell_height * 6) / 2 + 1;

        let mut day_name = Weekday::Sun;
        for i in 0..7 {
            let rect = Rect {
                x: (i as u16 * cell_width) + cx + offset_x,
                y: 2,
                width: cell_width,
                height: cell_height,
            };
            day_name = day_name.succ();
            buf.set_string(rect.x, rect.y, day_name.to_string(), AppStyles::Main.get());
        }

        for i in 0..6 {
            for j in 0..7 {
                let mut day: i64 = ((j + 1) + (7 * i) as i64) - state.start_day as i64;
                if day < 1 || day > state.num_of_days {
                    day = 0;
                }

                let border_type = if day == state.cur_day as i64 {
                    BorderType::Double
                } else {
                    BorderType::Plain
                };
                let border_style = if day == 0 {
                    Style::default().fg(Color::Black)
                } else if day == state.selected as i64 {
                    AppStyles::CalendarSelected.get()
                } else if day == state.cur_day as i64 {
                    AppStyles::CalendarCurDay.get()
                } else {
                    AppStyles::CalendarDeselected.get()
                };
                let symbols = BorderType::line_symbols(border_type);

                let rect = Rect {
                    x: (j as u16 * cell_width) + cx + offset_x,
                    y: (i as u16 * cell_height) + cy + offset_y,
                    width: cell_width,
                    height: cell_height,
                };

                // Sides
                if borders.intersects(Borders::LEFT) {
                    for y in rect.top()..rect.bottom() {
                        buf.get_mut(rect.left(), y)
                            .set_symbol(symbols.vertical)
                            .set_style(border_style);
                    }
                }
                if borders.intersects(Borders::TOP) {
                    for x in rect.left()..rect.right() {
                        buf.get_mut(x, rect.top())
                            .set_symbol(symbols.horizontal)
                            .set_style(border_style);
                    }
                }
                if borders.intersects(Borders::RIGHT) {
                    let x = rect.right() - 1;
                    for y in rect.top()..rect.bottom() {
                        buf.get_mut(x, y)
                            .set_symbol(symbols.vertical)
                            .set_style(border_style);
                    }
                }
                if borders.intersects(Borders::BOTTOM) {
                    let y = rect.bottom() - 1;
                    for x in rect.left()..rect.right() {
                        buf.get_mut(x, y)
                            .set_symbol(symbols.horizontal)
                            .set_style(border_style);
                    }
                }

                // Corners
                if borders.contains(Borders::RIGHT | Borders::BOTTOM) {
                    buf.get_mut(rect.right() - 1, rect.bottom() - 1)
                        .set_symbol(symbols.bottom_right)
                        .set_style(border_style);
                }
                if borders.contains(Borders::RIGHT | Borders::TOP) {
                    buf.get_mut(rect.right() - 1, rect.top())
                        .set_symbol(symbols.top_right)
                        .set_style(border_style);
                }
                if borders.contains(Borders::LEFT | Borders::BOTTOM) {
                    buf.get_mut(rect.left(), rect.bottom() - 1)
                        .set_symbol(symbols.bottom_left)
                        .set_style(border_style);
                }
                if borders.contains(Borders::LEFT | Borders::TOP) {
                    buf.get_mut(rect.left(), rect.top())
                        .set_symbol(symbols.top_left)
                        .set_style(border_style);
                }

                // write day number
                if day > 0 {
                    buf.set_string(rect.left(), rect.top(), day.to_string(), Style::default());
                }
            }
        }
    }
}

impl<'a> Calendar<'a> {
    pub fn new() -> Calendar<'a> {
        Calendar { data: vec![] }
    }
}

impl<'a> Widget for Calendar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = CalendarState::new();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
