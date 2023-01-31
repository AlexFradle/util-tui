use tui::widgets::{Block, Borders, Gauge};

use crate::styles::AppStyles;

pub struct ProgressBar {
    pub title: String,
    pub progress: u16,
}

impl ProgressBar {
    pub fn new(title: String, progress: u16) -> ProgressBar {
        ProgressBar { title, progress }
    }
    pub fn get_gauge(&self) -> Gauge {
        Gauge::default()
            .block(
                Block::default()
                    .title(&self.title[..])
                    .style(AppStyles::ProgressBar.get())
                    .borders(Borders::ALL),
            )
            .gauge_style(AppStyles::ProgressBar.get())
            .percent(self.progress)
    }
}
