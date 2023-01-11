use crate::{calendar::CalendarObj, list::ListObj, table::TableObj};

pub struct App<'a> {
    pub test_table: TableObj<'a>,
    pub test_list: ListObj<'a>,
    pub progress: u16,
    pub calendar: CalendarObj,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let t = TableObj::new(&["id", "name", "value"]);
        let l = ListObj::new();
        let c = CalendarObj::new();
        App {
            test_table: t,
            test_list: l,
            progress: 0,
            calendar: c,
        }
    }
}
