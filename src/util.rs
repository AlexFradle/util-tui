use log::info;
use std::{
    env,
    ops::{AddAssign, SubAssign},
    path::PathBuf,
    process::Command,
};

use figlet_rs::FIGfont;
use num_traits::{PrimInt, Signed};
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

pub fn clear_area(buf: &mut Buffer, area: Rect) {
    for x in area.left()..area.right() {
        for y in area.top()..area.bottom() {
            buf.get_mut(x, y).reset();
        }
    }
}

pub fn draw_ascii_string(
    buf: &mut Buffer,
    rect: Rect,
    string: &str,
    style: Style,
    center_x: bool,
    center_y: bool,
) -> (u16, u16) {
    let font = FIGfont::standard().unwrap();
    let fig = font.convert(string).unwrap();
    let mut x = 0;
    let mut char_positions: Vec<(u16, u16, String)> = Vec::new();
    for f in fig.characters {
        let (mut ox, mut oy) = (0, 0);
        for c in f.characters.join("\n").chars() {
            if c == '\n' {
                oy += 1;
                ox = 0;
            } else {
                ox += 1;
                let x_pos = rect.left() + ox + x;
                let y_pos = rect.top() + oy;
                if x_pos <= rect.left() + rect.width && y_pos <= rect.top() + rect.height {
                    char_positions.push((x_pos, y_pos, c.to_string()));
                }
            }
        }
        x += f.characters.iter().map(|x| x.len()).max().unwrap() as u16;
    }

    let min_x = char_positions.iter().map(|(x, _, _)| x).min().unwrap();
    let max_x = char_positions.iter().map(|(x, _, _)| x).max().unwrap();
    let min_y = char_positions.iter().map(|(_, y, _)| y).min().unwrap();
    let max_y = char_positions.iter().map(|(_, y, _)| y).max().unwrap();

    let mut offset_x = 0;
    if center_x {
        offset_x = rect.width / 2 - (max_x - min_x) / 2 - 1;
    }

    for (x, y, c) in &char_positions {
        buf.set_string(x + offset_x, *y, c, style);
    }

    (max_x - min_x, max_y - min_y)
}

fn run_command(command: impl Into<String>) -> String {
    let command = command.into();
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed");
    info!(
        "cmd: {} -> {}",
        command,
        String::from_utf8_lossy(&output.stdout).into_owned()
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn getcwd() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn get_calendar_events(year: i32, month: u32, num_of_days: i64) -> String {
    run_command(format!(
        "python {}/src/get_events.py {} {} {}",
        getcwd(),
        year,
        month,
        num_of_days
    ))
}

pub fn set_backlight(new_val: u16) {
    run_command(format!("light -S {}", new_val));
}

pub fn get_brightness() -> u16 {
    run_command("light -G | tr -d '\\n'")
        .parse::<f32>()
        .unwrap_or(0.0)
        .ceil() as u16
}

pub fn get_volume() -> u16 {
    // https://unix.stackexchange.com/a/89583
    run_command("awk -F\"[][]\" '/Left:/ { print $2 }' <(amixer sget Master) | tr -d '\\n%'")
        .parse::<u16>()
        .unwrap_or(0)
}

pub fn set_volume(new_val: u16) {
    run_command(format!("amixer sset Master {}%", new_val));
}

pub fn get_xresources() -> String {
    run_command("xresources_as_json")
}

/// Increment a value by an amount between upper and lower bounds
///
/// upper and lower must be the same type as the value input
pub fn generic_increment<T, U>(value: &mut T, lower: T, upper: T, amount: U)
where
    T: PrimInt + AddAssign<u32> + SubAssign<u32>,
    U: PrimInt + Signed,
{
    // TODO: find a way to go from signed generic to unsigned generic
    if *value > lower && amount.is_negative() {
        *value -= amount.abs().to_u32().unwrap();
    } else if *value < upper && amount.is_positive() {
        *value += amount.abs().to_u32().unwrap();
    }
}
