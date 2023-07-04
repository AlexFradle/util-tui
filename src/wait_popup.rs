use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{BorderType, Borders, Widget},
};

use crate::{
    styles::AppStyles,
    util::{centered_rect, clear_area, draw_rect_borders},
};

pub struct WaitPopup<'a> {
    old_buffer: &'a Buffer,
}

impl<'a> WaitPopup<'a> {
    pub fn new(old_buffer: &Buffer) -> WaitPopup {
        WaitPopup { old_buffer }
    }
}

impl<'a> Widget for WaitPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.merge(self.old_buffer);
        let rect = centered_rect(50, 50, area);
        clear_area(buf, rect);
        draw_rect_borders(
            buf,
            rect,
            Borders::ALL,
            BorderType::Plain,
            AppStyles::Main.get(),
        );
        let rect = Rect {
            x: rect.x + 1,
            y: rect.y + 1,
            width: rect.width - 2,
            height: rect.height - 2,
        };
        buf.set_string(
            rect.x,
            rect.y + rect.height / 2,
            format!("{:^1$}", "Please Wait", rect.width as usize),
            AppStyles::Main.get(),
        );
    }
}
