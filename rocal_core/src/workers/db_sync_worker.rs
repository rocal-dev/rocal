use proc_macro2::TokenStream;
use quote::quote;

pub fn build_db_sync_worker_struct() -> TokenStream {
    quote! {
        use serde::{Deserialize, Serialize};
        use web_sys::{Worker, WorkerOptions, WorkerType};

        pub struct DbSyncWorker<'a> {
            worker_path: &'a str,
            force: ForceType,
        }

        pub enum ForceType {
            #[allow(dead_code)]
            Local,
            Remote,
            None,
        }

        #[derive(Serialize, Deserialize)]
        struct Message<'a> {
            app_id: &'a str,
            directory_name: &'a str,
            file_name: &'a str,
            endpoint: &'a str,
            force: &'a str,
        }

        impl<'a> DbSyncWorker<'a> {
            pub fn new(worker_path: &'a str, force: ForceType) -> Self {
                DbSyncWorker { worker_path, force }
            }

            pub fn run(&self) {
                let options = WorkerOptions::new();
                options.set_type(WorkerType::Module);

                if let Ok(worker) = Worker::new_with_options(&self.worker_path, &options) {
                    let config = &crate::CONFIG;

                    let db = config.get_database().clone();

                    let force = match self.force {
                        ForceType::Local => "local",
                        ForceType::Remote => "remote",
                        ForceType::None => "none",
                    };

                    let message = Message {
                        app_id: config.get_app_id(),
                        directory_name: db.get_directory_name(),
                        file_name: db.get_file_name(),
                        endpoint: config.get_sync_server_endpoint(),
                        force,
                    };

                    if let Ok(value) = serde_wasm_bindgen::to_value(&message) {
                        let _ = worker.post_message(&value);
                    }
                }
            }
        }
    }
}
