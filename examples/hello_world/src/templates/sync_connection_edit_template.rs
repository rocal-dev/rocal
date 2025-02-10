use rocal::rocal_core::traits::{SharedRouter, Template};

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
        let mut html = String::from("<h1>DB sync connection</h1>");

        if let Some(connection) = data {
            html += &format!("<p>{} has been already connected.</p>", connection.get_id());
        } else {
            html += "
              <form action='/sync-connections'>
                <p><input type='text' name='id' placeholder='ID'></p>
                <p><input type='password' name='password' placeholder='Password'></p>
                <p><button type='submit'>Connect</button></p>
              </form>
            ";
        }

        html
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
