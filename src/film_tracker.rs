use serde::Deserialize;
use tui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    form::{Form, FormFieldStyle, FormState, TextField},
    styles::AppStyles,
    util::{draw_rect_borders, search_imdb},
};

#[derive(Deserialize, Debug)]
struct MovieSearchItem {
    title: String,
    imdb_id: String,
}

pub struct FilmTrackerState {
    search_form: FormState,
    movie_search_items: Vec<MovieSearchItem>,
}

impl FilmTrackerState {
    pub fn new() -> FilmTrackerState {
        let mut search_form = FormState::new();
        search_form.add_field(Box::new(TextField::new(
            "".to_owned(),
            false,
            FormFieldStyle::new("Title".to_owned()),
        )));
        FilmTrackerState {
            search_form,
            movie_search_items: vec![],
        }
    }

    pub async fn search_movie(&mut self, name: String) {
        let output = search_imdb(&name).await;
        self.movie_search_items = serde_json::from_str(&output).unwrap_or(vec![]);
    }
}

pub struct FilmTracker;

impl FilmTracker {
    pub fn new() -> FilmTracker {
        FilmTracker {}
    }
}

impl StatefulWidget for FilmTracker {
    type State = FilmTrackerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let search_form_rect = Rect {
            x: area.x,
            y: area.y,
            width: area.width / 4,
            height: 3,
        };
        Form.render(search_form_rect, buf, &mut state.search_form);
        for (p, m) in state.movie_search_items.iter().enumerate() {
            let p = p as u16;
            buf.set_string(area.x, area.y + 3 + p, &m.title, AppStyles::Main.get());
        }
    }
}
