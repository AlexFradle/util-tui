use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Spans,
    widgets::{BorderType, Borders, Widget},
};

use crate::{
    styles::AppStyles,
    util::{centered_rect, clear_area, draw_rect_borders},
};

pub struct Popup<'a> {
    pub pages: Vec<Spans<'a>>,
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_rect(75, 75, area);

        // clear area
        clear_area(buf, area);

        // draw border
        draw_rect_borders(
            buf,
            area,
            Borders::ALL,
            BorderType::Thick,
            AppStyles::Main.get(),
        );
    }
}

impl<'a> Popup<'a> {
    pub fn new() -> Popup<'a> {
        Popup { pages: vec![] }
    }
}
