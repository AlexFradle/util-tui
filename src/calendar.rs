use chrono::{DateTime, Datelike, Local, NaiveDate, Weekday};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct Calendar<'a> {
    pub data: Vec<(u8, &'a str)>,
    pub selected: u8,
    date_string: String,
    num_of_days: i64,
    start_day: u32,
    cur_day: u32,
}

impl<'a> Widget for Calendar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        let borders = Borders::ALL;

        let offset_x = (width - cell_width * 7) / 2;
        let offset_y = (height - cell_height * 6) / 2 + 1;

        let mut day_name = Weekday::Sun;
        for i in 0..7 {
            let rect = Rect {
                x: (i as u16 * cell_width) + cx + offset_x,
                y: 1,
                width: cell_width,
                height: cell_height,
            };
            day_name = day_name.succ();
            buf.set_string(rect.x, rect.y, day_name.to_string(), Style::default());
        }

        for i in 0..6 {
            for j in 0..7 {
                let mut day: i64 = ((j + 1) + (7 * i) as i64) - self.start_day as i64;
                if day < 1 || day > self.num_of_days {
                    day = 0;
                }

                let border_type = if day == self.cur_day as i64 {
                    BorderType::Double
                } else {
                    BorderType::Plain
                };
                let border_style = if day == 0 {
                    Style::default().fg(Color::Rgb(32, 32, 32))
                } else if day == self.cur_day as i64 {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
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
            cur_day,
        }
    }

    pub fn change_selected(&mut self, new_index: u8) {
        self.selected = new_index;
    }
}
