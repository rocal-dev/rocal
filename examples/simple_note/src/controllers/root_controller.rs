use crate::{
    models::note::Note, view_models::root_view_model::RootViewModel, views::root_view::RootView,
    CONFIG,
};
use rocal::rocal_core::traits::{Controller, SharedRouter};
use wasm_bindgen::JsValue;

pub struct RootController {
    router: SharedRouter,
    view: RootView,
}

impl Controller for RootController {
    type View = RootView;
    fn new(router: SharedRouter, view: Self::View) -> Self {
        RootController { router, view }
    }
}

impl RootController {
    #[rocal::action]
    pub fn index(&self, note_id: Option<i64>) {
        let db = CONFIG.get_database().clone();
        let result: Result<Vec<Note>, JsValue> =
            db.query("select id, title, body from notes").fetch().await;

        let notes = if let Ok(notes) = result {
            notes
        } else {
            vec![]
        };

        let note: Option<Note> = if let Some(note_id) = note_id {
            notes.iter().find(|note| note.id == note_id).cloned()
        } else {
            None
        };

        let vm = RootViewModel::new(note, notes);

        self.view.index(vm);
    }
}
