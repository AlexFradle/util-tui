use chrono::{DateTime, Datelike, Utc};
use log::info;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{BorderType, Borders, StatefulWidget},
};

use crate::{
    db::{MoneyTransaction, DB},
    form::{Form, FormField, FormFieldStyle, FormState},
    styles::AppStyles,
    util::{draw_rect_borders, generic_increment},
};

pub struct MoneyTracker;

#[derive(Debug)]
pub struct MoneyTrackerState {
    pub search_form: FormState,
    pub add_form: FormState,
    pub transactions: Vec<MoneyTransaction>,
    pub search_form_selected: bool,
    pub add_form_selected: bool,
    pub selected_transaction: u32,
}

impl MoneyTracker {
    pub fn new() -> MoneyTracker {
        MoneyTracker {}
    }
}

impl MoneyTrackerState {
    pub fn new() -> MoneyTrackerState {
        let mut search_form = FormState::new();
        search_form.add_field(FormField::Text {
            value: "".to_owned(),
            style: FormFieldStyle::new("Keyword".to_owned()),
        });
        search_form.add_field(FormField::Number {
            value: "".to_owned(),
            min: 0,
            max: 100000,
            is_float: true,
            style: FormFieldStyle::new("Min".to_owned()),
        });
        search_form.add_field(FormField::Number {
            value: "".to_owned(),
            min: 0,
            max: 100000,
            is_float: true,
            style: FormFieldStyle::new("Max".to_owned()),
        });

        let mut add_form = FormState::new();
        add_form.add_field(FormField::Text {
            value: "".to_owned(),
            style: FormFieldStyle::new("Title".to_owned()),
        });
        add_form.add_field(FormField::Number {
            value: "".to_owned(),
            min: 0,
            max: 100000,
            is_float: true,
            style: FormFieldStyle::new("Amount".to_owned()),
        });
        add_form.add_field(FormField::Text {
            value: "".to_owned(),
            style: FormFieldStyle::new("Details".to_owned()),
        });
        add_form.add_field(FormField::Date {
            value: "__ / __ / __".to_owned(),
            year: "".to_owned(),
            month: "".to_owned(),
            day: "".to_owned(),
            style: FormFieldStyle::new("Date".to_owned()),
        });
        MoneyTrackerState {
            search_form,
            add_form,
            transactions: vec![],
            search_form_selected: false,
            add_form_selected: false,
            selected_transaction: 0,
        }
    }

    pub fn increment_selected(&mut self, amount: i32) {
        generic_increment(
            &mut self.selected_transaction,
            0,
            self.transactions.len() as u32 - 1,
            amount,
        );
    }

    pub async fn submit_search_form(&mut self, db: &mut DB) {
        let fields = self.search_form.get_fields();
        let vals: Vec<String> = fields.iter().map(|f| f.get_value()).collect();
        match vals.as_slice() {
            [k, min, max] => {
                self.transactions.clear();
                self.transactions = db
                    .query_transactions(
                        k,
                        min.parse::<u32>().unwrap_or(0),
                        max.parse::<u32>().unwrap_or(u32::MAX),
                    )
                    .await;
                self.search_form.reset_fields();
                self.selected_transaction = 0;
            }
            [..] => {}
        };
    }

    pub async fn submit_add_form(&mut self, db: &mut DB) {
        let fields = self.add_form.get_fields();
        let vals: Vec<String> = fields.iter().map(|f| f.get_value()).collect();
        match vals.as_slice() {
            [t, a, d] => {
                let new_trans =
                    MoneyTransaction::new(t.clone(), a.parse::<f32>().unwrap(), Utc::now());
                db.add_transaction(&new_trans).await;
                self.add_form.reset_fields();
                self.submit_search_form(db).await;
            }
            [..] => {}
        };
    }

    pub fn select_search_form(&mut self) {
        self.search_form_selected = true;
        self.add_form_selected = false;
    }

    pub fn select_add_form(&mut self) {
        self.search_form_selected = false;
        self.add_form_selected = true;
    }

    pub fn select_transaction_list(&mut self) {
        self.search_form_selected = false;
        self.add_form_selected = false;
    }

    pub fn get_selected_transaction(&self) -> &MoneyTransaction {
        &self.transactions[self.selected_transaction as usize]
    }
}

impl StatefulWidget for MoneyTracker {
    type State = MoneyTrackerState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let right_pane = Rect {
            x: area.x + area.width / 3,
            y: area.y,
            width: area.width - area.width / 3,
            height: area.height,
        };
        let search_form_rect = Rect {
            x: area.x,
            y: area.y,
            width: area.width / 3,
            height: area.height / 3,
        };
        let add_form_rect = Rect {
            x: area.x,
            y: area.y + area.height / 3,
            width: area.width / 3,
            height: area.height - area.height / 3,
        };
        let search_form_style = if state.search_form_selected {
            AppStyles::Main.get()
        } else {
            AppStyles::Accent.get()
        };
        let add_form_style = if state.add_form_selected {
            AppStyles::Main.get()
        } else {
            AppStyles::Accent.get()
        };
        draw_rect_borders(
            buf,
            search_form_rect,
            Borders::all(),
            BorderType::Plain,
            search_form_style,
        );
        draw_rect_borders(
            buf,
            add_form_rect,
            Borders::all(),
            BorderType::Plain,
            add_form_style,
        );
        draw_rect_borders(
            buf,
            right_pane,
            Borders::all(),
            BorderType::Plain,
            if !state.search_form_selected && !state.add_form_selected {
                AppStyles::Main.get()
            } else {
                AppStyles::Accent.get()
            },
        );

        let search_form_rect = Rect {
            x: search_form_rect.x + 1,
            // + 2 for title
            y: search_form_rect.y + 2,
            width: search_form_rect.width - 2,
            height: search_form_rect.height - 2,
        };
        buf.set_string(
            search_form_rect.x,
            search_form_rect.y - 1,
            format!("{:^1$}", "Search", search_form_rect.width as usize),
            search_form_style.add_modifier(Modifier::BOLD),
        );
        let add_form_rect = Rect {
            x: add_form_rect.x + 1,
            // + 2 for title
            y: add_form_rect.y + 2,
            width: add_form_rect.width - 2,
            height: add_form_rect.height - 2,
        };
        buf.set_string(
            add_form_rect.x,
            add_form_rect.y - 1,
            format!("{:^1$}", "Add", add_form_rect.width as usize),
            add_form_style.add_modifier(Modifier::BOLD),
        );
        for field in state.add_form.get_fields_mut() {
            field.change_style_selected(add_form_style.clone());
        }
        for field in state.search_form.get_fields_mut() {
            field.change_style_selected(search_form_style.clone());
        }
        Form.render(search_form_rect, buf, &mut state.search_form);
        Form.render(add_form_rect, buf, &mut state.add_form);

        let right_pane = Rect {
            x: right_pane.x + 1,
            y: right_pane.y + 1,
            width: right_pane.width - 2,
            height: right_pane.height - 2,
        };

        let mut day_indexes: Vec<usize> = vec![];
        for (i, transaction) in state.transactions.iter().enumerate() {
            // first transaction always has different date
            if i == 0 {
                day_indexes.push(i);
            } else if state
                .transactions
                .get(i - 1)
                .unwrap()
                .date
                .num_days_from_ce()
                != transaction.date.num_days_from_ce()
            {
                day_indexes.push(i);
            }
        }

        let mut sub_heading_count = 0;
        for (i, transaction) in state.transactions.iter().enumerate() {
            let num_of_days = transaction.date.num_days_from_ce();
            let mut offset_y = right_pane.y + (1 * i as u16) + (1 * sub_heading_count);
            // can binary search because always in order
            if day_indexes.binary_search(&i).is_ok() {
                if state.get_selected_transaction().date.num_days_from_ce() == num_of_days {
                    buf.set_string(
                        right_pane.x,
                        offset_y,
                        format!(
                            "{:━^1$}",
                            transaction.date.format("%A %d %B %Y").to_string(),
                            right_pane.width as usize
                        ),
                        AppStyles::Main.get(),
                    );
                } else {
                    buf.set_string(
                        right_pane.x,
                        offset_y,
                        format!(
                            "{:-^1$}",
                            transaction.date.format("%A %d %B %Y").to_string(),
                            right_pane.width as usize
                        ),
                        AppStyles::Accent.get(),
                    );
                }
                sub_heading_count += 1;
                offset_y += 1;
            }
            let style = if i == state.selected_transaction as usize {
                AppStyles::Main.get()
            } else {
                AppStyles::Accent.get()
            };
            buf.set_string(
                right_pane.x,
                offset_y,
                format!("£{:<10}", format!("{:.2}", transaction.amount)),
                style,
            );
            // + 11 because left align width 10 + 1 for £ sign
            buf.set_string(right_pane.x + 11, offset_y, &transaction.title, style);
        }
    }
}
