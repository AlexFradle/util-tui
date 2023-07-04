use std::{collections::HashMap, fmt};

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
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
    title: String,
    borders: Borders,
    border_type: BorderType,
    selected_style: Style,
    unselected_style: Style,
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

#[derive(Debug, Clone)]
pub enum FormValue {
    Text(String),
    Integer(u32),
    Float(f32),
    Date(DateTime<Utc>),
}

impl FormValue {
    pub fn try_get_text_value(&self) -> Option<&String> {
        if let FormValue::Text(s) = self {
            return Some(&s);
        }
        None
    }

    pub fn try_get_text_value_mut(&mut self) -> Option<&mut String> {
        if let FormValue::Text(s) = self {
            return Some(s);
        }
        None
    }

    pub fn try_get_integer_value(&self) -> Option<&u32> {
        if let FormValue::Integer(i) = self {
            return Some(i);
        }
        None
    }

    pub fn try_get_integer_value_mut(&mut self) -> Option<&mut u32> {
        if let FormValue::Integer(i) = self {
            return Some(i);
        }
        None
    }

    pub fn try_get_float_value(&self) -> Option<&f32> {
        if let FormValue::Float(i) = self {
            return Some(i);
        }
        None
    }

    pub fn try_get_float_value_mut(&mut self) -> Option<&mut f32> {
        if let FormValue::Float(i) = self {
            return Some(i);
        }
        None
    }

    pub fn try_get_date_value(&self) -> Option<&DateTime<Utc>> {
        if let FormValue::Date(d) = self {
            return Some(d);
        }
        None
    }

    pub fn try_get_date_value_mut(&mut self) -> Option<&mut DateTime<Utc>> {
        if let FormValue::Date(d) = self {
            return Some(d);
        }
        None
    }
}

pub trait FormField {
    fn get_display_value(&self) -> String;
    fn get_internal_value(&self) -> &FormValue;
    fn get_default_value(&self) -> &FormValue;
    fn reset_value(&mut self);
    fn is_required(&self) -> bool;
    fn receive_input(&mut self, key: &KeyCode);
    fn get_style(&self) -> &FormFieldStyle;
    fn change_style_selected(&mut self, new_style: Style);
}

macro_rules! form_field_access_funcs {
    () => {
        fn get_internal_value(&self) -> &FormValue {
            &self.value
        }

        fn get_default_value(&self) -> &FormValue {
            &self.default_value
        }

        fn is_required(&self) -> bool {
            self.is_required
        }

        fn get_style(&self) -> &FormFieldStyle {
            &self.style
        }

        fn change_style_selected(&mut self, new_style: Style) {
            self.style.selected_style = new_style;
        }
    };
}

#[derive(Debug)]
pub struct TextField {
    value: FormValue,
    default_value: FormValue,
    is_required: bool,
    style: FormFieldStyle,
}

impl TextField {
    pub fn new(default_value: String, is_required: bool, style: FormFieldStyle) -> TextField {
        TextField {
            value: FormValue::Text(default_value.clone()),
            default_value: FormValue::Text(default_value),
            is_required,
            style,
        }
    }
}

impl FormField for TextField {
    form_field_access_funcs!();

    fn get_display_value(&self) -> String {
        self.value.try_get_text_value().unwrap().clone()
    }

    fn reset_value(&mut self) {
        self.value = self.default_value.clone();
    }

    fn receive_input(&mut self, key: &KeyCode) {
        let current_value = self.value.try_get_text_value_mut().unwrap();
        match key {
            KeyCode::Backspace => {
                if current_value.len() > 0 {
                    current_value.truncate(current_value.len() - 1);
                }
            }
            KeyCode::Char(char) => {
                current_value.push_str(&char.to_string());
            }
            _ => {}
        }
    }
}

pub struct IntegerField {
    value: FormValue,
    default_value: FormValue,
    min: u32,
    max: u32,
    is_required: bool,
    style: FormFieldStyle,
}

impl IntegerField {
    pub fn new(
        default_value: u32,
        min: u32,
        max: u32,
        is_required: bool,
        style: FormFieldStyle,
    ) -> IntegerField {
        IntegerField {
            value: FormValue::Integer(default_value),
            default_value: FormValue::Integer(default_value),
            min,
            max,
            is_required,
            style,
        }
    }
}

impl FormField for IntegerField {
    form_field_access_funcs!();

    fn get_display_value(&self) -> String {
        self.value.try_get_integer_value().unwrap().to_string()
    }

    fn reset_value(&mut self) {
        self.value = self.default_value.clone();
    }

    fn receive_input(&mut self, key: &KeyCode) {
        let current_value = self.value.try_get_integer_value_mut().unwrap();
        match key {
            KeyCode::Char(num @ '0'..='9') => {
                *current_value = (*current_value * 10) + num.to_digit(10).unwrap();
            }
            KeyCode::Backspace => {
                if *current_value > 0 {
                    *current_value /= 10;
                }
            }
            _ => {}
        }
    }
}

pub struct FloatField {
    value: FormValue,
    default_value: FormValue,
    display_value: String,
    min: f32,
    max: f32,
    is_required: bool,
    style: FormFieldStyle,
}

impl FloatField {
    pub fn new(
        default_value: f32,
        min: f32,
        max: f32,
        is_required: bool,
        style: FormFieldStyle,
    ) -> FloatField {
        FloatField {
            value: FormValue::Float(default_value),
            default_value: FormValue::Float(default_value),
            display_value: "".to_owned(),
            min,
            max,
            is_required,
            style,
        }
    }
}

impl FormField for FloatField {
    form_field_access_funcs!();

    fn get_display_value(&self) -> String {
        self.display_value.clone()
    }

    fn reset_value(&mut self) {
        self.value = self.default_value.clone();
        self.display_value = "".to_owned();
    }

    fn receive_input(&mut self, key: &KeyCode) {
        let current_value = self.value.try_get_float_value_mut().unwrap();
        match key {
            KeyCode::Char(num @ ('0'..='9' | '.')) => {
                let mut new_value = num.to_string();
                if *current_value > 0. {
                    new_value = format!("{}{}", self.display_value, num);
                }
                if let Ok(n) = new_value.parse::<f32>() {
                    if n <= self.max {
                        self.display_value = new_value;
                        *current_value = n;
                    }
                }
            }
            KeyCode::Backspace => {
                if self.display_value.len() > 0 {
                    self.display_value.truncate(self.display_value.len() - 1);
                    if self.display_value.len() > 0 {
                        *current_value = self.display_value.parse::<f32>().unwrap();
                    } else {
                        *current_value = 0.;
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct DateField {
    value: FormValue,
    default_value: FormValue,
    day: String,
    month: String,
    year: String,
    is_required: bool,
    style: FormFieldStyle,
}

impl DateField {
    pub fn new(
        default_value: DateTime<Utc>,
        is_required: bool,
        style: FormFieldStyle,
    ) -> DateField {
        DateField {
            value: FormValue::Date(default_value.clone()),
            default_value: FormValue::Date(default_value),
            day: format!("{:0>2}", default_value.day()),
            month: format!("{:0>2}", default_value.month()),
            year: default_value.year().to_string()[2..4].to_string(),
            is_required,
            style,
        }
    }
}

impl FormField for DateField {
    form_field_access_funcs!();

    fn get_display_value(&self) -> String {
        format!("{:_<2} / {:_<2} / {:_<2}", self.day, self.month, self.year)
    }

    fn reset_value(&mut self) {
        self.value = self.default_value.clone();
        let date = self.value.try_get_date_value().unwrap();
        self.day = format!("{:0>2}", date.day());
        self.month = format!("{:0>2}", date.month());
        self.year = date.year().to_string()[2..4].to_string();
    }

    fn receive_input(&mut self, key: &KeyCode) {
        let current_value = self.value.try_get_date_value_mut().unwrap();
        let num_of_days: HashMap<u32, u32> = HashMap::from([
            (1, 31),
            (2, 29),
            (3, 31),
            (4, 30),
            (5, 31),
            (6, 30),
            (7, 31),
            (8, 31),
            (9, 30),
            (10, 31),
            (11, 30),
            (12, 31),
        ]);
        match key {
            KeyCode::Char(num @ '0'..='9') => {
                if self.day.len() < 2 {
                    self.day.push_str(&num.to_string());
                    let day = self.day.parse::<u32>().unwrap();
                    if day > 31 {
                        self.day.truncate(self.day.len() - 1);
                    }
                } else if self.month.len() < 2 {
                    self.month.push_str(&num.to_string());
                    if self.month.len() == 2 {
                        let day = self.day.parse::<u32>().unwrap();
                        let month = self.month.parse::<u32>().unwrap();
                        if month <= 12 {
                            if day > *num_of_days.get(&month).unwrap() {
                                self.month.truncate(self.month.len() - 1);
                            }
                        } else {
                            self.month.truncate(self.month.len() - 1);
                        }
                    }
                } else if self.year.len() < 2 {
                    self.year.push_str(&num.to_string());
                    if self.year.len() == 2 {
                        let day = self.day.parse::<u32>().unwrap();
                        let month = self.month.parse::<u32>().unwrap();
                        let year = self.year.parse::<u32>().unwrap();
                        let new_date = NaiveDate::from_ymd_opt(2000 + year as i32, month, day);
                        if let Some(d) = new_date {
                            *current_value = DateTime::from_utc(
                                d.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                                Utc,
                            );
                        } else {
                            self.year.truncate(self.year.len() - 1);
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if self.year.len() > 0 {
                    self.year.truncate(self.year.len() - 1);
                } else if self.month.len() > 0 {
                    self.month.truncate(self.month.len() - 1);
                } else if self.day.len() > 0 {
                    self.day.truncate(self.day.len() - 1);
                }
            }
            _ => {}
        }
    }
}
// ----------------------------------------------------------------------------

pub struct FormState {
    fields: Vec<Box<dyn FormField>>,
    selected_field: u32,
}

impl fmt::Debug for FormState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormState [fields: [len = {}], selected_field: {}]",
            self.fields.len(),
            self.selected_field
        )
    }
}

impl FormState {
    pub fn new() -> FormState {
        FormState {
            fields: vec![],
            selected_field: 0,
        }
    }

    pub fn add_field(&mut self, new_field: Box<dyn FormField>) {
        self.fields.push(new_field);
    }

    pub fn get_fields(&self) -> &Vec<Box<dyn FormField>> {
        &self.fields
    }

    pub fn get_fields_mut(&mut self) -> &mut Vec<Box<dyn FormField>> {
        &mut self.fields
    }

    pub fn get_selected_field(&self) -> &Box<dyn FormField> {
        &self.fields[self.selected_field as usize]
    }

    pub fn get_selected_field_mut(&mut self) -> &mut Box<dyn FormField> {
        &mut self.fields[self.selected_field as usize]
    }

    pub fn reset_fields(&mut self) {
        self.selected_field = 0;
        for field in &mut self.fields.iter_mut() {
            field.reset_value();
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
        self.fields[self.selected_field as usize].receive_input(key);
    }
}

pub struct Form;

impl Form {
    pub fn new() -> Form {
        Form {}
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

            let value = field.get_display_value();

            buf.set_string(
                field_area.x + 1,
                field_area.y,
                format!(
                    "{}{}",
                    if field.is_required() { "*" } else { "" },
                    &style.title
                ),
                field_style,
            );
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
