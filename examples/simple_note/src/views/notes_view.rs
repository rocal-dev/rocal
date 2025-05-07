use rocal::rocal_core::traits::{SharedRouter, View};

pub struct NotesView {
    router: SharedRouter,
}

impl View for NotesView {
    fn new(router: SharedRouter) -> Self {
        Self { router }
    }
}
