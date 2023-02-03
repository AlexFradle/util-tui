use chrono::{DateTime, Datelike, FixedOffset, Local, Month, NaiveDate, Timelike, Weekday};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use tui::layout::{Constraint, Direction, Layout};
use tui::symbols::line;
use tui::text::Span;
use tui::widgets::{Paragraph, Wrap};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{BorderType, Borders, StatefulWidget, Widget},
};

use crate::styles::AppStyles;
use crate::styles::{ACCENT_COLOR, MAIN_COLOR};
use crate::util::{draw_rect_borders, get_calendar_events};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CalendarEvent {
    pub start: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct CalendarState {
    pub data: HashMap<u32, Vec<CalendarEvent>>,
    pub selected_day: u32,
    pub cur_day: u32,
    pub cur_month: Month,
    pub cur_year: i32,
    pub num_of_days: i64,
    pub start_day: u32,
    pub show_popup: bool,
    pub selected_event: u32,
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
        let data = CalendarState::get_data(cur_year, cur_month, num_of_days);

        CalendarState {
            data,
            selected_day: cur_day,
            cur_day,
            cur_month: Month::from_u32(cur_month).unwrap(),
            cur_year,
            num_of_days,
            start_day,
            show_popup: false,
            selected_event: 0,
        }
    }

    fn get_data(year: i32, month: u32, num_of_days: i64) -> HashMap<u32, Vec<CalendarEvent>> {
        let output = get_calendar_events(year, month, num_of_days);
        let data: Vec<CalendarEvent> = serde_json::from_str(&output).unwrap_or(vec![]);
        let mut days_data: HashMap<u32, Vec<CalendarEvent>> = HashMap::new();
        for event in data {
            let day = event.start.day();
            days_data.entry(day).or_default();
            days_data.get_mut(&day).unwrap().push(event);
        }
        return days_data;
    }

    fn set_data(&mut self) {
        self.data = CalendarState::get_data(
            self.cur_year,
            self.cur_month.number_from_month(),
            self.num_of_days,
        );
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
        self.set_data();
    }

    pub fn increment_selected_day(&mut self, amount: i32) {
        if self.selected_day > 1 && amount.is_negative() {
            self.selected_day -= amount.abs() as u32;
        } else if self.selected_day < self.num_of_days as u32 && amount.is_positive() {
            self.selected_day += amount.abs() as u32;
        }
    }

    pub fn popup_toggle(&mut self) {
        self.show_popup = !self.show_popup;
        self.selected_event = 0;
    }

    pub fn increment_selected_event(&mut self, amount: i32) {
        let empty_vec: Vec<CalendarEvent> = Vec::new();
        let num_of_events = self
            .data
            .get(&self.selected_day)
            .unwrap_or(&empty_vec)
            .len();
        if self.selected_event > 0 && amount.is_negative() {
            self.selected_event -= amount.abs() as u32;
        } else if self.selected_event < num_of_events as u32 - 1 && amount.is_positive() {
            self.selected_event += amount.abs() as u32;
        }
    }
}

pub struct Calendar;

impl Calendar {
    pub fn new() -> Calendar {
        Calendar {}
    }
}

impl StatefulWidget for Calendar {
    type State = CalendarState;

    fn render(self, r_area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let l = Layout::default()
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .direction(Direction::Horizontal)
            .split(r_area);
        let (c_area, t_area) = (l[0], l[1]);
        draw_rect_borders(
            buf,
            c_area,
            Borders::ALL,
            BorderType::Plain,
            AppStyles::Main.get(),
        );
        // + 1 to avoid border
        let cx = c_area.left() + 1;
        // + 1 to avoid border
        let cy = c_area.top() + 1;
        // - 2 for top and bottom border
        let width = c_area.width - 2;
        // - 2 for top and bottom border, - 1 for day line
        let height = c_area.height - 2 - 2;
        let cell_width = width / 7;
        let cell_height = height / 6;

        // draw month name and year at top
        let month_str = format!("{} {}", state.cur_month.name(), state.cur_year);
        buf.set_string(
            cx + width / 2 - (month_str.len() / 2) as u16,
            cy,
            month_str,
            AppStyles::Main.get().add_modifier(Modifier::BOLD),
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
                } else if day == state.selected_day as i64 {
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
                    if day == state.selected_day as i64 || day == state.cur_day as i64 {
                        for x in rect.left()..rect.right() {
                            buf.get_mut(x, rect.top())
                                .set_symbol(symbols.horizontal)
                                .set_style(border_style);
                        }
                    } else {
                        for i in 0..3 {
                            buf.get_mut(rect.left() + i, rect.top())
                                .set_symbol(symbols.horizontal)
                                .set_style(border_style);
                        }
                        let diff = rect.right() - rect.left();
                        for i in (diff - 3)..diff {
                            buf.get_mut(rect.left() + i, rect.top())
                                .set_symbol(symbols.horizontal)
                                .set_style(border_style);
                        }
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
                    if day == state.selected_day as i64 || day == state.cur_day as i64 {
                        for x in rect.left()..rect.right() {
                            buf.get_mut(x, y)
                                .set_symbol(symbols.horizontal)
                                .set_style(border_style);
                        }
                    } else {
                        for i in 0..3 {
                            buf.get_mut(rect.left() + i, y)
                                .set_symbol(symbols.horizontal)
                                .set_style(border_style);
                        }
                        let diff = rect.right() - rect.left();
                        for i in (diff - 3)..diff {
                            buf.get_mut(rect.left() + i, y)
                                .set_symbol(symbols.horizontal)
                                .set_style(border_style);
                        }
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

                if day > 0 {
                    // write day number
                    buf.set_string(
                        rect.left() + rect.width / 2
                            - (day.to_string().len() as f32 / 2.0).ceil() as u16,
                        rect.top(),
                        if day.to_string().len() == 1 {
                            format!("0{}", day.to_string())
                        } else {
                            day.to_string()
                        },
                        if day == state.cur_day as i64 {
                            AppStyles::CalendarCurDay.get().fg(
                                if day == state.selected_day as i64 {
                                    MAIN_COLOR
                                } else {
                                    ACCENT_COLOR
                                },
                            )
                        } else if day == state.selected_day as i64 {
                            AppStyles::CalendarSelected.get()
                        } else {
                            AppStyles::CalendarDeselected.get()
                        },
                    );
                    match state.data.get(&(day as u32)) {
                        Some(v) => v.iter().enumerate().for_each(|(i, event)| {
                            buf.set_string(
                                rect.left() + 1 + i as u16,
                                rect.top() + 1,
                                "ﱢ",
                                AppStyles::Main.get(),
                            );
                        }),
                        None => {}
                    }
                }
            }
        }

        // draw timeline area borders
        draw_rect_borders(
            buf,
            t_area,
            Borders::ALL,
            BorderType::Plain,
            AppStyles::Main.get(),
        );

        // get area inset by 1 due to borders
        let t_area = Rect {
            x: t_area.x + 1,
            y: t_area.y + 1,
            width: t_area.width - 2,
            height: t_area.height - 2,
        };

        buf.set_string(
            // - 3 because "Events" is 6 chars long
            t_area.x + t_area.width / 2 - 3,
            t_area.y,
            "Events",
            AppStyles::Main.get().add_modifier(Modifier::BOLD),
        );

        // origin coords, y + 2 for heading and line break
        let (ox, oy) = (t_area.x, t_area.y + 2);

        // draw timeline data
        let empty_vec: Vec<CalendarEvent> = Vec::new();
        let event_list = state.data.get(&state.selected_day).unwrap_or(&empty_vec);
        for (i, event) in event_list.iter().enumerate() {
            let i = i as u16;
            let height = 5;
            let gap = 3;
            let bar_x_offset = 5;
            let y_pos = oy + (i * height) + (gap * i);
            // draw start time
            buf.set_string(
                ox,
                y_pos,
                format!("{}", event.start.format("%H:%M")),
                AppStyles::Main.get().add_modifier(Modifier::BOLD),
            );
            // draw end time
            buf.set_string(
                ox,
                y_pos + (height - 1),
                format!("{}", event.end.format("%H:%M")),
                AppStyles::Main.get().add_modifier(Modifier::BOLD),
            );
            // draw event bars
            for j in 0..height {
                buf.set_string(
                    ox + bar_x_offset,
                    y_pos + j,
                    line::THICK_VERTICAL,
                    AppStyles::Main.get(),
                );
            }
            // draw gap bars
            if i as usize != event_list.len() - 1 {
                let midpoint = (gap as f32 / 2.0).ceil() as u16;
                for j in 0..gap {
                    // draw bar
                    buf.set_string(
                        ox + bar_x_offset,
                        y_pos + j + height,
                        line::VERTICAL,
                        AppStyles::Accent.get(),
                    );

                    // draw time difference text
                    if j == midpoint - 1 {
                        let text = format!(
                            "{} Hour(s)",
                            event_list[i as usize + 1].start.hour() - event.end.hour()
                        );
                        let line_width = t_area.width - bar_x_offset - text.len() as u16;
                        let lines = "-".repeat(line_width as usize / 2);
                        buf.set_string(
                            ox + bar_x_offset,
                            y_pos + j + height,
                            format!("{}{}{}", lines, text, lines),
                            AppStyles::Accent.get(),
                        );
                    }
                }
            }
            // draw title
            let p = Paragraph::new(Span::raw(&event.title))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            let event_text_area = Rect {
                x: ox + bar_x_offset + 1,
                y: oy + (i * height) + (gap * i),
                width: t_area.width - bar_x_offset - 1,
                height,
            };
            p.render(event_text_area, buf);
            //             buf.set_string(
            //                 event_text_area.x,
            //                 event_text_area.y + event_text_area.height - 1,
            //                 "TIME",
            //                 Style::default(),
            //             );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_state() {
        let cs = CalendarState::new();
        println!("{:?}", cs);
    }
}
