use log::info;

use crate::{
    calendar::CalendarState,
    clock::ClockState,
    db::DB,
    grade_tracker::GradeTrackerState,
    money_tracker::MoneyTrackerState,
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
    pub db: DB,
    pub money_state: MoneyTrackerState,
}

impl App {
    pub async fn new() -> App {
        let mut db = DB::new().await;
        db.run_migrations().await;
        let mut money_state = MoneyTrackerState::new();
        money_state.transactions = db.get_all_transactions().await;

        App {
            brightness: get_brightness(),
            volume: get_volume(),
            calendar_state: CalendarState::new(),
            grade_state: GradeTrackerState::new(),
            clock_state: ClockState::new(),
            cur_screen: Screen::DashboardScreen,
            db,
            money_state,
        }
    }
}
