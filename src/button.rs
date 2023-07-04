use tui::{
    style::Style,
    widgets::{BorderType, Borders, Widget},
};

use crate::{styles::AppStyles, util::draw_rect_borders};

pub struct Button {
    text: String,
    is_selected: bool,
}

impl Button {
    pub fn new(text: String, is_selected: bool) -> Button {
        Button { text, is_selected }
    }
}

impl Widget for Button {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        draw_rect_borders(
            buf,
            area,
            Borders::ALL,
            BorderType::Plain,
            if self.is_selected {
                AppStyles::ButtonSelected.get()
            } else {
                AppStyles::ButtonDeselected.get()
            },
        );
        buf.set_string(area.x, area.y, self.text, AppStyles::Backgroud.get());
    }
}
