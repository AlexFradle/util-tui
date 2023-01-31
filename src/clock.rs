use std::{fs::File, io::Write};

use chrono::{DateTime, Local, Timelike};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{BorderType, Borders, StatefulWidget, Widget},
};

use figlet_rs::FIGfont;

use crate::{
    styles::AppStyles,
    util::{centered_rect, draw_ascii_string, draw_rect_borders},
};

pub struct ClockState {
    pub time: DateTime<Local>,
}

impl ClockState {
    pub fn new() -> ClockState {
        ClockState { time: Local::now() }
    }
}

pub struct Clock {
    pub with_border: bool,
}

impl StatefulWidget for Clock {
    type State = ClockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let cur_time = state.time.time().format("%H:%M").to_string();
        let cur_date = state.time.format("%A %d %B %Y").to_string();
        draw_rect_borders(
            buf,
            area,
            if self.with_border {
                Borders::ALL
            } else {
                Borders::NONE
            },
            BorderType::Plain,
            AppStyles::Main.get(),
        );
        let area = Rect {
            x: area.left() + 1,
            y: area.top() + 1,
            width: area.width - 2,
            height: area.height - 2,
        };
        let (text_width, text_height) =
            draw_ascii_string(buf, area, &cur_time, AppStyles::Main.get(), true, false);
        buf.set_string(1, text_height + 2, cur_date, Style::default());
    }
}

impl Clock {
    pub fn new(with_border: bool) -> Clock {
        Clock { with_border }
    }
}
