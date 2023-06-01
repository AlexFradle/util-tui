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
    pub style: Style,
}

impl FormFieldStyle {
    pub fn new(title: String) -> FormFieldStyle {
        FormFieldStyle {
            title,
            borders: Borders::ALL,
            border_type: BorderType::Plain,
            style: AppStyles::Main.get(),
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
        value: i64,
        min: i64,
        max: i64,
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
}

impl FormField {
    pub fn get_style(&self) -> &FormFieldStyle {
        match self {
            FormField::Text { style, .. } => style,
            FormField::Number { style, .. } => style,
            FormField::Select { style, .. } => style,
            FormField::Radio { style, .. } => style,
            FormField::Checkbox { style, .. } => style,
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            FormField::Text { value, .. } => value.clone(),
            FormField::Number { value, .. } => value.to_string(),
            FormField::Select { value, .. } => value.to_string(),
            FormField::Radio { value, .. } => value.to_string(),
            FormField::Checkbox { value, .. } => value.to_string(),
        }
    }
}

// ----------------------------------------------------------------------------

pub struct Form;

#[derive(Debug)]
pub struct FormState {
    fields: Vec<FormField>,
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
            selected_field: 0,
        }
    }

    pub fn add_field(&mut self, new_field: FormField) {
        self.fields.push(new_field);
    }

    pub fn get_selected_field(&self) -> &FormField {
        &self.fields[self.selected_field as usize]
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
                KeyCode::Enter => {}
                KeyCode::Char(char) => {
                    value.push_str(&char.to_string());
                }
                _ => {}
            },
            FormField::Number { value, .. } => {}
            FormField::Select { value, .. } => {}
            FormField::Radio { value, .. } => {}
            FormField::Checkbox { value, .. } => {}
        };
    }
}

impl StatefulWidget for Form {
    type State = FormState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.fields.is_empty() {
            return;
        }

        let total_height = state.fields.len() * 3 + (state.fields.len() - 1) + 3;
        if total_height as u16 > area.height {
            return;
        }

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

            let text_style = if is_selected {
                AppStyles::Main.get()
            } else {
                AppStyles::Accent.get()
            };

            draw_rect_borders(
                buf,
                field_area,
                style.borders,
                style.border_type,
                text_style,
            );

            let value = field.get_value();

            buf.set_string(field_area.x + 1, field_area.y, &style.title, text_style);
            buf.set_string(field_area.x + 1, field_area.y + 1, &value, text_style);

            if is_selected {
                buf.set_string(
                    field_area.x + 1 + value.len() as u16,
                    field_area.y + 1,
                    symbols::block::FULL,
                    style.style,
                );
            }
        }
        let button_rect = Rect {
            x: area.x + (area.width - 2) / 4,
            y: area.y + area.height - 3,
            width: (area.width - 2) / 2,
            height: 3,
        };
        draw_rect_borders(
            buf,
            button_rect,
            Borders::ALL,
            BorderType::Plain,
            AppStyles::Main.get(),
        );
        buf.set_string(
            button_rect.x + (button_rect.width - 2) / 2 - 1,
            button_rect.y + 1,
            "Submit",
            AppStyles::Main.get(),
        );
    }
}

// ----------------------------------------------------------------------------

#[test]
fn test_form() {
    println!("test");
}
