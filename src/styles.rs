use tui::style::{Color, Modifier, Style};

const GREEN: Color = Color::Rgb(0, 255, 0);
const ACCENT: Color = Color::Rgb(0, 95, 0);

pub enum AppStyles {
    Main,
    TableHeader,
    TableHighlight,
    ProgressBar,
    ListItem,
}

impl AppStyles {
    pub fn get(&self) -> Style {
        match self {
            AppStyles::Main => Style::default().fg(GREEN),
            AppStyles::TableHeader => Style::default().bg(GREEN).fg(Color::Black),
            AppStyles::TableHighlight => Style::default().bg(ACCENT),
            AppStyles::ProgressBar => Style::default().fg(GREEN).bg(Color::Black),
            AppStyles::ListItem => Style::default().fg(Color::Red),
        }
    }
}
