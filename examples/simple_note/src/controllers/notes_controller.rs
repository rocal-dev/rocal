use rocal::rocal_core::traits::{Controller, SharedRouter};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::{models::note_id::NoteId, views::notes_view::NotesView, CONFIG};

pub struct NotesController {
    router: SharedRouter,
    view: NotesView,
}

impl Controller for NotesController {
    type View = NotesView;

    fn new(router: SharedRouter, view: Self::View) -> Self {
        Self { router, view }
    }
}

impl NotesController {
    #[rocal::action]
    pub fn create(&self, title: Option<String>, body: Option<String>) {
        let db = CONFIG.get_database().clone();

        let result: Result<Vec<NoteId>, JsValue> = if let (Some(title), Some(body)) = (title, body)
        {
            db.query("insert into notes(title, body) values ($1, $2) returning id;")
                .bind(title)
                .bind(body)
                .fetch()
                .await
        } else {
            db.query("insert into notes(title, body) values (null, null) returning id;")
                .fetch()
                .await
        };

        let result = match result {
            Ok(result) => result,
            Err(err) => {
                console::error_1(&err);
                return;
            }
        };

        if let Some(note_id) = result.get(0) {
            self.router
                .borrow()
                .redirect(&format!("/?note_id={}", &note_id.id))
                .await;
        } else {
            console::error_1(&"Could not add a new note".into());
        }
    }

    #[rocal::action]
    pub fn update(&self, note_id: i64, title: String, body: String) {
        let db = CONFIG.get_database().clone();

        let result = db
            .query("update notes set title = $1, body = $2 where id = $3;")
            .bind(title)
            .bind(body)
            .bind(note_id)
            .execute()
            .await;

        match result {
            Ok(_) => {
                self.router
                    .borrow()
                    .redirect(&format!("/?note_id={}", &note_id))
                    .await;
            }
            Err(err) => {
                console::error_1(&err);
                return;
            }
        };
    }

    #[rocal::action]
    pub fn delete(&self, note_id: i64) {
        let db = CONFIG.get_database().clone();

        let result = db
            .query("delete from notes where id = $1;")
            .bind(note_id)
            .execute()
            .await;

        match result {
            Ok(_) => {
                self.router.borrow().redirect("/").await;
            }
            Err(err) => {
                console::error_1(&err);
            }
        };
    }
}
