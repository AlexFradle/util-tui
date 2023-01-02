use crate::styles::AppStyles;
use tui::{
    layout::Constraint,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

pub struct TableObj<'a> {
    pub items: Vec<Vec<&'a str>>,
    pub headers: Vec<&'a str>,
    pub state: TableState,
    pub num_of_cols: usize,
}

impl<'a> TableObj<'a> {
    pub fn new(headers: &'a [&str]) -> TableObj<'a> {
        TableObj {
            items: vec![],
            headers: Vec::from(headers),
            state: TableState::default(),
            num_of_cols: headers.len(),
        }
    }

    pub fn get_table(&self) -> Table<'a> {
        let header_cells = self
            .headers
            .iter()
            .map(|h| Cell::from(*h).style(AppStyles::TableHeader.get()));
        let header = Row::new(header_cells)
            .style(AppStyles::Main.get())
            .height(1);

        let rows = self.items.iter().map(|item| {
            // count height by calculating number of newline chars
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;

            // create cell using str, * to go from &&str -> &str
            let cells = item.iter().map(|c| Cell::from(*c));

            // return row of cells with correct height
            Row::new(cells).height(height as u16)
        });
        Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(AppStyles::TableHighlight.get())
            .highlight_symbol("->")
            .widths(&[
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
    }

    pub fn add_row(&mut self, row_data: &'a [&str]) -> Result<(), &str> {
        if row_data.len() != self.num_of_cols {
            return Err("could not add row");
        }

        self.items.push(Vec::from(row_data));
        Ok(())
    }

    pub fn remove_row(&mut self, index: usize) -> Result<(), &str> {
        if index > self.items.len() - 1 {
            return Err("index too big");
        }
        self.items.remove(index);
        Ok(())
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
}
