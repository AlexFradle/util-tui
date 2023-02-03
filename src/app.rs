use crate::{
    calendar::CalendarState,
    clock::ClockState,
    grade_tracker::GradeTrackerState,
    list::ListObj,
    screens::Screen,
    table::TableObj,
    util::{get_brightness, get_volume},
};

pub struct App<'a> {
    pub test_table: TableObj<'a>,
    pub test_list: ListObj<'a>,
    pub brightness: u16,
    pub volume: u16,
    pub calendar_state: CalendarState,
    pub grade_state: GradeTrackerState,
    pub clock_state: ClockState,
    pub cur_screen: Screen,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let t = TableObj::new(&["id", "name", "value"]);
        let l = ListObj::new();
        App {
            test_table: t,
            test_list: l,
            brightness: get_brightness(),
            volume: get_volume(),
            calendar_state: CalendarState::new(),
            grade_state: GradeTrackerState::new(),
            clock_state: ClockState::new(),
            cur_screen: Screen::DashboardScreen,
        }
    }
}
