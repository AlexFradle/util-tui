use std::fs;

use serde::Deserialize;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{BorderType, Borders, StatefulWidget, Widget},
};

use crate::{
    styles::AppStyles,
    util::{draw_rect_borders, getcwd},
};

#[derive(Deserialize, Debug)]
pub struct Grade {
    pub name: String,
    pub percentage: f32,
    pub weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub grades: Vec<Grade>,
}

#[derive(Debug)]
pub struct GradeTrackerState {
    pub data: Vec<Module>,
}

impl GradeTrackerState {
    pub fn new() -> GradeTrackerState {
        GradeTrackerState {
            data: GradeTrackerState::get_data(),
        }
    }

    fn get_data() -> Vec<Module> {
        let str_data = fs::read_to_string(format!("{}/src/grades.json", getcwd()))
            .unwrap_or(String::from("[]"));
        serde_json::from_str(&str_data).unwrap_or(vec![])
    }
}

pub struct GradeTracker;

impl StatefulWidget for GradeTracker {
    type State = GradeTrackerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        draw_rect_borders(
            buf,
            area,
            Borders::ALL,
            BorderType::Plain,
            AppStyles::Main.get(),
        );

        let area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        let max_height = area.height / 4;
        let height = area.height / state.data.len() as u16;
        let height = if height > max_height {
            max_height
        } else {
            height
        };

        for (i, module) in state.data.iter().enumerate() {
            let i = i as u16;
            let rect = Rect {
                x: area.x,
                y: area.y + height * i,
                width: area.width,
                height,
            };
            let (ox, oy) = (area.x + 1, area.y + (height * i) + 1);
            // draw border
            draw_rect_borders(
                buf,
                rect,
                Borders::ALL,
                BorderType::Plain,
                AppStyles::Main.get(),
            );
            // draw module name
            buf.set_string(ox, oy, &module.name, Style::default());
        }
    }
}

impl GradeTracker {
    pub fn new() -> GradeTracker {
        GradeTracker {}
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_json() {
        let g = GradeTrackerState::new();
        println!("{:?}", g.data);
    }
}
