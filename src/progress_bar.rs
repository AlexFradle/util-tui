use tui::widgets::{Block, Borders, Gauge};

use crate::styles::AppStyles;

pub struct ProgressBar {
    pub progress: u16,
}

impl ProgressBar {
    pub fn new(progress: u16) -> ProgressBar {
        ProgressBar { progress }
    }
    pub fn get_gauge(&self) -> Gauge {
        Gauge::default()
            .block(Block::default().title("Progress"))
            .gauge_style(AppStyles::ProgressBar.get())
            .percent(self.progress)
    }
}
