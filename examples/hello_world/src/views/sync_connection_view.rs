use rocal::rocal_core::traits::{SharedRouter, Template, View};

use crate::{
    models::sync_connection::SyncConnection,
    templates::sync_connection_edit_template::SyncConnectionEditTemplate,
};

pub struct SyncConnectionView {
    router: SharedRouter,
}

impl View for SyncConnectionView {
    fn new(router: SharedRouter) -> Self {
        Self { router }
    }
}

impl SyncConnectionView {
    pub fn edit(&self, connection: Option<SyncConnection>) {
        let template = SyncConnectionEditTemplate::new(self.router.clone());
        template.render(connection);
    }
}
