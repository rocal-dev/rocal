use rocal::rocal_core::{
    enums::request_method::RequestMethod,
    traits::{Controller, SharedRouter},
};

use crate::{
    repositories::sync_connection_repository::SyncConnectionRepository,
    views::sync_connection_view::SyncConnectionView, DbSyncWorker, ForceType, CONFIG,
};

pub struct SyncConnectionsController {
    router: SharedRouter,
    view: SyncConnectionView,
}

impl Controller for SyncConnectionsController {
    type View = SyncConnectionView;

    fn new(router: SharedRouter, view: Self::View) -> Self {
        Self { router, view }
    }
}

impl SyncConnectionsController {
    #[rocal::action]
    pub async fn edit(&self) {
        let repo = SyncConnectionRepository::new(CONFIG.get_database());

        if let Ok(connection) = repo.get().await {
            self.view.edit(connection);
        } else {
            self.view.edit(None);
        }
    }

    #[rocal::action]
    pub async fn connect(&self, id: String, password: String) {
        let repo = SyncConnectionRepository::new(CONFIG.get_database());

        match repo.create(&id, &password).await {
            Ok(_) => {
                self.router
                    .borrow()
                    .resolve(RequestMethod::Get, "/", None)
                    .await;

                let db_sync_worker = DbSyncWorker::new("./js/db_sync_worker.js", ForceType::Remote);
                db_sync_worker.run();
            }
            Err(err) => web_sys::console::error_1(&err),
        }
    }
}
