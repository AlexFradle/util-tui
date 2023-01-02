use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::styles::AppStyles;

pub struct ListObj<'a> {
    pub items: Vec<&'a str>,
    pub state: ListState,
}

impl<'a> ListObj<'a> {
    pub fn new() -> ListObj<'a> {
        ListObj {
            items: vec!["a", "b", "c", "d", "e"],
            state: ListState::default(),
        }
    }

    pub fn get_list(&self) -> List<'a> {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|text| ListItem::new(Span::styled(*text, AppStyles::ListItem.get())))
            .collect();
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ")
    }

    pub fn add_item(&mut self, item: &'a str) {
        self.items.push(item);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
