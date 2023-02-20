use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{BorderType, Borders, StatefulWidget},
};

use crate::util::draw_rect_borders;

// // ----------------------------------------------------------------------------
//
// trait FormInput<T: FormState> {
//     fn recieve_input(&self, input: String, state: &mut T);
// }
//
// trait FormState {}
//
// impl<T> FormInput<T> for T
// where
//     T: FormState,
// {
//     fn recieve_input(&self, input: String, state: &mut T) {}
// }
//
// // ----------------------------------------------------------------------------
//
// pub struct Form {
//     fields: Vec<Box<dyn FormInput<dyn FormState>>>,
//     states: Vec<Box<dyn FormState>>,
// }
//
// impl Form {
//     pub fn new() -> Form {
//         Form {
//             fields: vec![],
//             states: vec![],
//         }
//     }
//
//     pub fn add_text_field(
//         mut self,
//         title: String,
//         borders: Borders,
//         border_type: BorderType,
//         style: Style,
//     ) -> Form {
//         self.fields
//             .push(Box::new(TextInput::new(title, borders, border_type, style)));
//         self.states.push(Box::new(TextInputState::new()));
//         self
//     }
// }
//
// // ----------------------------------------------------------------------------
//
// struct TextInput {
//     title: String,
//     borders: Borders,
//     border_type: BorderType,
//     style: Style,
// }
//
// struct TextInputState {
//     current_string: String,
// }
//
// impl TextInputState {
//     pub fn new() -> TextInputState {
//         TextInputState {
//             current_string: "".to_owned(),
//         }
//     }
// }
//
// impl FormState for TextInputState {}
//
// impl TextInput {
//     pub fn new(
//         title: String,
//         borders: Borders,
//         border_type: BorderType,
//         style: Style,
//     ) -> TextInput {
//         TextInput {
//             title,
//             borders,
//             border_type,
//             style,
//         }
//     }
// }
//
// impl FormInput<TextInputState> for TextInput {
//     fn recieve_input(&self, input: String, state: &mut TextInputState) {
//         state.current_string = input;
//     }
// }
//
// impl StatefulWidget for TextInput {
//     type State = TextInputState;
//
//     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         draw_rect_borders(buf, area, self.borders, self.border_type, self.style);
//         buf.set_string(0, 0, &self.title, self.style);
//     }
// }
//
// // ----------------------------------------------------------------------------
//
// struct NumberInput;
//
// // ----------------------------------------------------------------------------
//
// struct Dropdown;
