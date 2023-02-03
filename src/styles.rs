use tui::style::{Color, Modifier, Style};

pub const MAIN_COLOR: Color = Color::Green;
pub const ACCENT_COLOR: Color = Color::Rgb(0, 95, 0);
pub const BACKGROUND_COLOR: Color = Color::Black;

pub enum AppStyles {
    Main,
    Accent,
    TableHeader,
    TableHighlight,
    ProgressBar,
    ListItem,
    CalendarCurDay,
    CalendarSelected,
    CalendarDeselected,
}

impl AppStyles {
    pub fn get(&self) -> Style {
        match self {
            AppStyles::Main => Style::default().fg(MAIN_COLOR),
            AppStyles::Accent => Style::default().fg(ACCENT_COLOR),
            AppStyles::TableHeader => Style::default().bg(MAIN_COLOR).fg(BACKGROUND_COLOR),
            AppStyles::TableHighlight => Style::default().bg(ACCENT_COLOR),
            AppStyles::ProgressBar => Style::default().fg(MAIN_COLOR).bg(BACKGROUND_COLOR),
            AppStyles::ListItem => Style::default().fg(Color::Red),
            AppStyles::CalendarCurDay => Style::default().fg(ACCENT_COLOR),
            AppStyles::CalendarSelected => Style::default().fg(MAIN_COLOR),
            AppStyles::CalendarDeselected => Style::default().fg(ACCENT_COLOR),
        }
    }
}
