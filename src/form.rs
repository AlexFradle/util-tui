use crossterm::event::KeyCode;
use log::info;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols,
    widgets::{BorderType, Borders, StatefulWidget},
};

use crate::{
    styles::AppStyles,
    util::{draw_rect_borders, generic_increment},
};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct FormFieldStyle {
    pub title: String,
    pub borders: Borders,
    pub border_type: BorderType,
    pub selected_style: Style,
    pub unselected_style: Style,
}

impl FormFieldStyle {
    pub fn new(title: String) -> FormFieldStyle {
        FormFieldStyle {
            title,
            borders: Borders::ALL,
            border_type: BorderType::Plain,
            selected_style: AppStyles::Main.get(),
            unselected_style: AppStyles::Accent.get(),
        }
    }
}

#[derive(Debug)]
pub enum FormField {
    Text {
        value: String,
        style: FormFieldStyle,
    },
    Number {
        value: String,
        min: u32,
        max: u32,
        is_float: bool,
        style: FormFieldStyle,
    },
    Select {
        value: usize,
        style: FormFieldStyle,
    },
    Radio {
        value: usize,
        style: FormFieldStyle,
    },
    Checkbox {
        value: bool,
        style: FormFieldStyle,
    },
    Date {
        value: String,
        year: String,
        month: String,
        day: String,
        style: FormFieldStyle,
    },
}

impl FormField {
    pub fn get_style(&self) -> &FormFieldStyle {
        match self {
            FormField::Text { style, .. } => style,
            FormField::Number { style, .. } => style,
            FormField::Select { style, .. } => style,
            FormField::Radio { style, .. } => style,
            FormField::Checkbox { style, .. } => style,
            FormField::Date { style, .. } => style,
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            FormField::Text { value, .. } => value.clone(),
            FormField::Number { value, .. } => value.to_string(),
            FormField::Select { value, .. } => value.to_string(),
            FormField::Radio { value, .. } => value.to_string(),
            FormField::Checkbox { value, .. } => value.to_string(),
            FormField::Date { value, .. } => value.clone(),
        }
    }

    pub fn change_style_selected(&mut self, new_style: Style) {
        let mut style = match self {
            FormField::Text { style, .. } => style,
            FormField::Number { style, .. } => style,
            FormField::Select { style, .. } => style,
            FormField::Radio { style, .. } => style,
            FormField::Checkbox { style, .. } => style,
            FormField::Date { style, .. } => style,
        };
        (*style).selected_style = new_style;
    }
}

// ----------------------------------------------------------------------------

pub struct Form;

#[derive(Debug)]
pub struct FormState {
    fields: Vec<FormField>,
    initial_values: Vec<String>,
    pub selected_field: u32,
}

impl Form {
    pub fn new() -> Form {
        Form {}
    }
}

impl FormState {
    pub fn new() -> FormState {
        FormState {
            fields: vec![],
            initial_values: vec![],
            selected_field: 0,
        }
    }

    pub fn add_field(&mut self, new_field: FormField) {
        self.initial_values.push(new_field.get_value());
        self.fields.push(new_field);
    }

    pub fn get_fields(&self) -> &Vec<FormField> {
        &self.fields
    }

    pub fn get_fields_mut(&mut self) -> &mut Vec<FormField> {
        &mut self.fields
    }

    pub fn get_selected_field(&self) -> &FormField {
        &self.fields[self.selected_field as usize]
    }

    pub fn reset_fields(&mut self) {
        self.selected_field = 0;
        for (i, field) in &mut self.fields.iter_mut().enumerate() {
            let v = self.initial_values.get(i).unwrap();
            match field {
                FormField::Text { value, .. } => {
                    *value = v.clone();
                }
                FormField::Number { value, .. } => {
                    *value = v.clone();
                }
                FormField::Select { value, .. } => {
                    *value = v.parse::<usize>().unwrap();
                }
                FormField::Radio { value, .. } => {
                    *value = v.parse::<usize>().unwrap();
                }
                FormField::Checkbox { value, .. } => {
                    *value = v.parse::<bool>().unwrap();
                }
                FormField::Date { value, .. } => {
                    *value = v.clone();
                }
            };
        }
    }

    pub fn increment_selected(&mut self, amount: i32) {
        generic_increment(
            &mut self.selected_field,
            0,
            self.fields.len() as u32 - 1,
            amount,
        );
    }

    pub fn send_input(&mut self, key: &KeyCode) {
        match &mut self.fields[self.selected_field as usize] {
            FormField::Text { value, .. } => match key {
                KeyCode::Backspace => {
                    if value.len() > 0 {
                        value.truncate(value.len() - 1);
                    }
                }
                KeyCode::Char(char) => {
                    value.push_str(&char.to_string());
                }
                _ => {}
            },
            FormField::Number {
                value,
                min,
                max,
                is_float,
                ..
            } => match key {
                KeyCode::Char(num @ ('0'..='9' | '.')) => {
                    let new_value = format!("{}{}", value, num);
                    // dont allow decimal point in non float
                    if !*is_float && *num == '.' {
                        return;
                    }
                    if let Ok(n) = new_value.parse::<f32>() {
                        if n <= *max as f32 {
                            *value = new_value;
                        }
                    }
                }
                KeyCode::Backspace => {
                    if value.len() > 0 {
                        value.truncate(value.len() - 1);
                    }
                }
                _ => {}
            },
            FormField::Select { value, .. } => {}
            FormField::Radio { value, .. } => {}
            FormField::Checkbox { value, .. } => {}
            FormField::Date {
                value,
                year,
                month,
                day,
                ..
            } => {
                match key {
                    KeyCode::Char(num @ '0'..='9') => {
                        if day.len() < 2 {
                            day.push_str(&num.to_string());
                        } else if month.len() < 2 {
                            month.push_str(&num.to_string());
                        } else if year.len() < 2 {
                            year.push_str(&num.to_string());
                        }
                    }
                    _ => {}
                };
                *value = format!("{:_<2} / {:_<2} / {:_<2}", day, month, year);
            }
        };
    }
}

impl StatefulWidget for Form {
    type State = FormState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.fields.is_empty() {
            return;
        }

        //         let total_height = state.fields.len() * 3 + (state.fields.len() - 1) + 3;
        //         if total_height as u16 > area.height {
        //             return;
        //         }

        for (i, field) in state.fields.iter().enumerate() {
            let i = i as u16;
            let style = field.get_style();
            let field_area = Rect {
                x: area.x,
                y: area.y + (3 * i),
                width: area.width,
                height: 3,
            };

            let is_selected = i == state.selected_field as u16;
            let field_style = if is_selected {
                style.selected_style
            } else {
                style.unselected_style
            };

            draw_rect_borders(
                buf,
                field_area,
                style.borders,
                style.border_type,
                field_style,
            );

            let value = field.get_value();

            buf.set_string(field_area.x + 1, field_area.y, &style.title, field_style);
            buf.set_string(field_area.x + 1, field_area.y + 1, &value, field_style);

            if is_selected {
                buf.set_string(
                    field_area.x + 1 + value.len() as u16,
                    field_area.y + 1,
                    symbols::block::FULL,
                    style.selected_style,
                );
            }
        }
        //         let button_rect = Rect {
        //             x: area.x + (area.width - 2) / 4,
        //             y: area.y + area.height - 3,
        //             width: (area.width - 2) / 2,
        //             height: 3,
        //         };
        //         draw_rect_borders(
        //             buf,
        //             button_rect,
        //             Borders::ALL,
        //             BorderType::Plain,
        //             AppStyles::Main.get(),
        //         );
        //         buf.set_string(
        //             button_rect.x + (button_rect.width - 2) / 2 - 1,
        //             button_rect.y + 1,
        //             "Submit",
        //             AppStyles::Main.get(),
        //         );
    }
}

// ----------------------------------------------------------------------------
