use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{BorderType, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    styles::AppStyles,
    util::{centered_rect, clear_area, draw_rect_borders},
};

pub struct Popup<'a> {
    pub pages: Vec<Spans<'a>>,
    pub width_percent: u16,
    pub height_percent: u16,
    last_line_on_border: bool,
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_rect(self.width_percent, self.height_percent, area);

        // remove all text in area to draw popup
        clear_area(buf, area);

        // draw border
        draw_rect_borders(
            buf,
            area,
            Borders::ALL,
            BorderType::Thick,
            AppStyles::Main.get(),
        );

        let area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        // make paragraph
        let t = if self.last_line_on_border && self.pages.len() > 1 {
            let text = String::from(self.pages.last().unwrap().clone());
            buf.set_string(
                area.x + area.width / 2 - text.len() as u16 / 2,
                area.y + area.height,
                text,
                Style::default(),
            );
            // get all elems except last
            self.pages.split_last().unwrap().1.to_vec()
        } else {
            self.pages
        };
        let p = Paragraph::new(t)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });
        p.render(area, buf);
    }
}

impl<'a> Popup<'a> {
    pub fn new(
        pages: Vec<Spans<'a>>,
        width_percent: u16,
        height_percent: u16,
        last_line_on_border: bool,
    ) -> Popup<'a> {
        Popup {
            pages,
            width_percent,
            height_percent,
            last_line_on_border,
        }
    }
}
