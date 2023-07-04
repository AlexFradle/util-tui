use std::collections::HashMap;

use serde::Deserialize;
use tui::style::{Color, Modifier, Style};

use crate::util::get_xresources;

lazy_static! {
    pub static ref COLORS: ColorData = ColorData::new();
}

pub struct ColorData {
    pub main: Color,
    pub accent: Color,
    pub background: Color,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum XValues {
    Nest(HashMap<String, String>),
    Generic(String),
}

impl ColorData {
    fn hex_to_color(h: impl Into<String>) -> Color {
        let h: Vec<u32> = h
            .into()
            .chars()
            .skip(1)
            .map(|c| c.to_digit(16).unwrap())
            .collect();
        Color::Rgb(
            (h[0] * 16 + h[1]) as u8,
            (h[2] * 16 + h[3]) as u8,
            (h[4] * 16 + h[5]) as u8,
        )
    }

    pub fn new() -> ColorData {
        // get colors from i3wm xresources values
        let x: HashMap<String, XValues> = serde_json::from_str(&get_xresources()).unwrap();

        let default = HashMap::from_iter([
            ("main".to_owned(), "#00ff00".to_owned()),
            ("accent".to_owned(), "#005f00".to_owned()),
        ]);
        let fallback = XValues::Nest(default.clone());

        let values = match x.get("i3wm").unwrap_or(&fallback) {
            XValues::Nest(v) => v,
            XValues::Generic(_) => &default,
        };

        ColorData {
            main: ColorData::hex_to_color(values.get("main").unwrap()),
            accent: ColorData::hex_to_color(values.get("accent").unwrap()),
            background: Color::Black,
        }
    }
}

pub enum AppStyles {
    Main,
    Accent,
    Backgroud,
    InvertedMain,
    ProgressBar,
    CalendarCurDay,
    CalendarSelected,
    CalendarDeselected,
    TitleText,
    TitleTextDeactivated,
    ButtonSelected,
    ButtonDeselected,
}

impl AppStyles {
    pub fn get(&self) -> Style {
        match self {
            AppStyles::Main => Style::default().fg(COLORS.main),
            AppStyles::Accent => Style::default().fg(COLORS.accent),
            AppStyles::Backgroud => Style::default().fg(COLORS.background),
            AppStyles::InvertedMain => Style::default().fg(COLORS.background).bg(COLORS.main),
            AppStyles::ProgressBar => Style::default().fg(COLORS.main).bg(COLORS.background),
            AppStyles::CalendarCurDay => Style::default().fg(COLORS.accent),
            AppStyles::CalendarSelected => Style::default().fg(COLORS.main),
            AppStyles::CalendarDeselected => Style::default().fg(COLORS.accent),
            AppStyles::TitleText => Style::default()
                .fg(COLORS.main)
                .add_modifier(Modifier::BOLD),
            AppStyles::TitleTextDeactivated => Style::default()
                .fg(COLORS.accent)
                .add_modifier(Modifier::BOLD),
            AppStyles::ButtonDeselected => Style::default().bg(COLORS.accent).fg(COLORS.accent),
            AppStyles::ButtonSelected => Style::default().bg(COLORS.main).fg(COLORS.main),
        }
    }
}
