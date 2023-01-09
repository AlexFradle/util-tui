use tui::style::{Color, Modifier, Style};

const ACCENT: Color = Color::Rgb(0, 95, 0);

pub enum AppStyles {
    Main,
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
            AppStyles::Main => Style::default().fg(Color::Green),
            AppStyles::TableHeader => Style::default().bg(Color::Green).fg(Color::Black),
            AppStyles::TableHighlight => Style::default().bg(ACCENT),
            AppStyles::ProgressBar => Style::default().fg(Color::Green).bg(Color::Black),
            AppStyles::ListItem => Style::default().fg(Color::Red),
            AppStyles::CalendarCurDay => Style::default().fg(ACCENT),
            AppStyles::CalendarSelected => Style::default().fg(Color::Green),
            AppStyles::CalendarDeselected => Style::default().fg(ACCENT),
        }
    }
}
