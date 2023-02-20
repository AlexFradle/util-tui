use crate::{
    calendar::CalendarState,
    clock::ClockState,
    grade_tracker::GradeTrackerState,
    screens::Screen,
    util::{get_brightness, get_volume},
};

pub struct App {
    pub brightness: u16,
    pub volume: u16,
    pub calendar_state: CalendarState,
    pub grade_state: GradeTrackerState,
    pub clock_state: ClockState,
    pub cur_screen: Screen,
}

impl App {
    pub fn new() -> App {
        App {
            brightness: get_brightness(),
            volume: get_volume(),
            calendar_state: CalendarState::new(),
            grade_state: GradeTrackerState::new(),
            clock_state: ClockState::new(),
            cur_screen: Screen::DashboardScreen,
        }
    }
}
