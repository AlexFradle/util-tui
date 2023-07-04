use log::info;

use crate::{
    calendar::CalendarState,
    clock::ClockState,
    db::DB,
    film_tracker::FilmTrackerState,
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
    pub film_state: FilmTrackerState,
}

impl App {
    pub async fn new() -> App {
        let mut db = DB::new().await;
        db.run_migrations().await;

        App {
            brightness: get_brightness(),
            volume: get_volume(),
            calendar_state: CalendarState::new().await,
            grade_state: GradeTrackerState::new(),
            clock_state: ClockState::new(),
            cur_screen: Screen::DashboardScreen,
            db,
            money_state: MoneyTrackerState::new(),
            film_state: FilmTrackerState::new(),
        }
    }
}
