use crate::{list::ListObj, table::TableObj};

pub struct App<'a> {
    pub test_table: TableObj<'a>,
    pub test_list: ListObj<'a>,
    pub progress: u16,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let t = TableObj::new(&["id", "name", "value"]);
        let l = ListObj::new();
        App {
            test_table: t,
            test_list: l,
            progress: 0,
        }
    }
}
