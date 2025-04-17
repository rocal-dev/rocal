use rocal::rocal_core::traits::{SharedRouter, View};

pub struct EmptyView {
    router: SharedRouter,
}

impl View for EmptyView {
    fn new(router: SharedRouter) -> Self {
        Self { router }
    }
}
