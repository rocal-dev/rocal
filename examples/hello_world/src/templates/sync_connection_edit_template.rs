use rocal::{
    rocal_core::traits::{SharedRouter, Template},
    view,
};

use crate::models::sync_connection::SyncConnection;

pub struct SyncConnectionEditTemplate {
    router: SharedRouter,
}

impl Template for SyncConnectionEditTemplate {
    type Data = Option<SyncConnection>;

    fn new(router: SharedRouter) -> Self {
        Self { router }
    }

    fn body(&self, data: Self::Data) -> String {
        view! {
            <h1>{"DB sync connection"}</h1>
            if let Some(connection) = data {
                <p>{{ connection.get_id() }} {" has been already connected."}</p>
            } else {
                <form action="/sync-connections">
                    <p><input type="text" name="id" placeholder="ID"></p>
                    <p><input type="password" name="password" placeholder="Password"></p>
                    <p><button type="submit">{"Connect"}</button></p>
                </form>
            }

        }
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
