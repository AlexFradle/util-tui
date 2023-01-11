use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{BorderType, Borders},
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn draw_rect_borders(
    buf: &mut Buffer,
    rect: Rect,
    borders: Borders,
    border_type: BorderType,
    border_style: Style,
) {
    let symbols = BorderType::line_symbols(border_type);
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
}
