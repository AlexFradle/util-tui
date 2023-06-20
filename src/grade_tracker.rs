use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
};

use log::info;
use serde::{Deserialize, Serialize};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{BorderType, Borders, StatefulWidget},
};

use crate::{
    form::{
        FloatField, Form, FormField, FormFieldStyle, FormState, FormValue, IntegerField, TextField,
    },
    styles::AppStyles,
    util::{centered_rect, clear_area, draw_rect_borders, generic_increment, getcwd},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Grade {
    pub name: String,
    pub percentage: f32,
    pub weight: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Module {
    pub name: String,
    pub grades: Vec<Grade>,
}

#[derive(Debug)]
pub struct GradeTrackerState {
    pub data: Vec<Module>,
    pub selected: u32,
    pub show_form: bool,
    pub form_state: FormState,
}

impl GradeTrackerState {
    pub fn new() -> GradeTrackerState {
        let mut form_state = FormState::new();
        form_state.add_field(Box::new(TextField::new(
            "".to_owned(),
            true,
            FormFieldStyle::new("Title".to_owned()),
        )));
        form_state.add_field(Box::new(FloatField::new(
            0.,
            0.,
            100.,
            true,
            FormFieldStyle::new("Percentage".to_owned()),
        )));
        form_state.add_field(Box::new(FloatField::new(
            0.,
            0.,
            100.,
            true,
            FormFieldStyle::new("Weight".to_owned()),
        )));
        GradeTrackerState {
            data: GradeTrackerState::get_data(),
            selected: 0,
            show_form: false,
            form_state,
        }
    }

    pub fn submit_form(&mut self) {
        let fields = self.form_state.get_fields();
        let vals: Vec<&FormValue> = fields.iter().map(|f| f.get_internal_value()).collect();
        match vals.as_slice() {
            [title, percentage, weight] => {
                let grade = Grade {
                    name: title.try_get_text_value().unwrap().clone(),
                    percentage: *percentage.try_get_float_value().unwrap(),
                    weight: *weight.try_get_float_value().unwrap(),
                };
                if self.write_data().is_ok() {
                    self.add_grade_to_selected(grade);
                    self.form_state.reset_fields();
                }
            }
            [..] => {}
        }
    }

    fn get_data() -> Vec<Module> {
        let str_data = fs::read_to_string(format!("{}/src/grades.json", getcwd()))
            .unwrap_or(String::from("[]"));
        serde_json::from_str(&str_data).unwrap_or(vec![])
    }

    fn add_grade_to_selected(&mut self, grade: Grade) {
        self.data[self.selected as usize].grades.push(grade);
    }

    fn write_data(&self) -> io::Result<()> {
        let file = match File::options()
            .write(true)
            .truncate(true)
            .open(format!("{}/src/grades.json", getcwd()))
        {
            Ok(file) => file,
            Err(_) => panic!("no file grades.json"),
        };
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn increment_selected(&mut self, amount: i32) {
        generic_increment(&mut self.selected, 0, self.data.len() as u32 - 1, amount);
    }

    pub fn toggle_form(&mut self) {
        self.show_form = !self.show_form;
    }
}

pub struct GradeTracker;

impl GradeTracker {
    pub fn new() -> GradeTracker {
        GradeTracker {}
    }
}

impl StatefulWidget for GradeTracker {
    type State = GradeTrackerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let min_height: u16 = 3;
        let tall_height = area.height - min_height * (state.data.len() - 1) as u16;
        let bar_height = 3;
        let num_of_columns = 2;
        // -2 for side borders, other - for bar side borders
        let bar_width = (area.width - 2 - (num_of_columns * 2)) / num_of_columns;
        // -1 for module name first line
        let num_of_rows = (tall_height - 1) / bar_height;

        for (i, module) in state.data.iter().enumerate() {
            let i = i as u16;

            // calculate total percentage
            let total_percent: f32 = module
                .grades
                .iter()
                .map(|g| g.percentage * (g.weight / 100.0))
                .sum();

            // average
            let mean_avg: f32 = module.grades.iter().map(|g| g.percentage).sum();
            let mean_avg = mean_avg / module.grades.len() as f32;

            // weighted average
            let weighted_avg: f32 = module.grades.iter().map(|g| g.weight).sum();
            let weighted_avg = total_percent / (weighted_avg / 100.0);

            let oy = if i > state.selected as u16 {
                tall_height + (i - 1) * min_height
            } else {
                i * min_height
            };

            let is_selected = i == state.selected as u16;

            let rect = Rect {
                x: area.x,
                y: area.y + oy,
                width: area.width,
                height: if is_selected { tall_height } else { min_height },
            };

            let (ox, oy) = (area.x + 1, oy + 1);

            // draw border
            draw_rect_borders(
                buf,
                rect,
                Borders::ALL,
                BorderType::Plain,
                if is_selected {
                    AppStyles::Main.get()
                } else {
                    AppStyles::Accent.get()
                },
            );
            // draw module name
            buf.set_string(
                ox,
                oy,
                &module.name,
                if is_selected {
                    AppStyles::TitleText.get()
                } else {
                    AppStyles::TitleTextDeactivated.get()
                },
            );
            let total_text = format!(
                "Overall: {}.{}%",
                total_percent.trunc(),
                (total_percent.fract() * 100.0).round()
            );
            let mean_text = format!(
                "Mean: {}.{}%",
                mean_avg.trunc(),
                (mean_avg.fract() * 100.0).round()
            );
            let weighted_text = format!(
                "Weighted: {}.{}%",
                weighted_avg.trunc(),
                (weighted_avg.fract() * 100.0).round()
            );
            let stats_style = if is_selected {
                AppStyles::Main.get()
            } else {
                AppStyles::Accent.get()
            };
            // draw total percent
            buf.set_string(
                ox + rect.width - 2 - total_text.len() as u16,
                oy,
                &total_text,
                stats_style,
            );
            // draw mean text
            buf.set_string(
                ox + rect.width - 2 - 1 - total_text.len() as u16 - mean_text.len() as u16,
                oy,
                &mean_text,
                stats_style,
            );
            // draw weighted text
            buf.set_string(
                ox + rect.width
                    - 2
                    - 2
                    - total_text.len() as u16
                    - mean_text.len() as u16
                    - weighted_text.len() as u16,
                oy,
                &weighted_text,
                stats_style,
            );
            // draw module grades
            if is_selected {
                for (j, grade) in module.grades.iter().enumerate() {
                    let j = j as u16;
                    let bar_rect = Rect {
                        x: ox + bar_width * (j / num_of_rows),
                        y: oy + 1 + bar_height * (j % num_of_rows),
                        width: bar_width,
                        height: bar_height,
                    };
                    draw_rect_borders(
                        buf,
                        bar_rect,
                        Borders::ALL,
                        BorderType::Plain,
                        AppStyles::Main.get(),
                    );
                    let highlighted_rect = Rect {
                        x: bar_rect.x + 1,
                        y: bar_rect.y + 1,
                        width: bar_rect.width
                            - 2
                            - ((bar_width as f32 - 2.0) * ((100.0 - grade.percentage) / 100.0))
                                as u16,
                        height: bar_rect.height - 2,
                    };
                    buf.set_string(
                        bar_rect.x + 1,
                        bar_rect.y + 1,
                        &grade.name,
                        AppStyles::Main.get(),
                    );
                    let percent_text = format!("{}%", grade.percentage);
                    buf.set_string(
                        bar_rect.x - 1 + bar_rect.width - percent_text.len() as u16,
                        bar_rect.y + 1,
                        percent_text,
                        AppStyles::Main.get(),
                    );
                    buf.set_style(highlighted_rect, AppStyles::InvertedMain.get());
                }
            }
        }
        if state.show_form {
            let area = centered_rect(50, 50, area);
            clear_area(buf, area);
            draw_rect_borders(
                buf,
                area,
                Borders::ALL,
                BorderType::Thick,
                AppStyles::Main.get(),
            );
            let title_text = " Enter New Assessment ";
            buf.set_string(
                area.x + ((area.width - 2) / 2) - (title_text.len() as u16 / 2),
                area.y,
                title_text,
                AppStyles::Main.get(),
            );
            let area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width - 2,
                height: area.height - 2,
            };
            Form.render(area, buf, &mut state.form_state);
        }
    }
}
