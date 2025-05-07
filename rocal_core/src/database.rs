use proc_macro2::TokenStream;
use quote::quote;

pub fn build_database_struct() -> TokenStream {
    quote! {
        use js_sys::Promise;
        use wasm_bindgen::{JsCast, JsValue};
        use serde::de::DeserializeOwned;
        use serde_wasm_bindgen::from_value;

        pub struct Database {
            directory_name: String,
            file_name: String,
        }

        impl Database {
            pub fn new(directory_name: String, file_name: String) -> Self {
                Database {
                    directory_name,
                    file_name,
                }
            }

            pub fn get_directory_name(&self) -> &str {
                &self.directory_name
            }

            pub fn get_file_name(&self) -> &str {
                &self.file_name
            }

            pub fn get_name(&self) -> String {
                format!("{}/{}", self.directory_name, self.file_name)
            }

            pub fn query(&self, query: &str) -> Query {
                Query::new(&self.get_name(), query)
            }
        }

        struct Query {
            db: String,
            query: String,
            bindings: Vec<JsValue>,
        }

        impl Query {
            fn new(db: &str, query: &str) -> Self {
                Self {
                    db: db.to_string(),
                    query: query.to_string(),
                    bindings: vec![],
                }
            }

            fn bind<T>(&mut self, bind: T) -> &mut Self
            where
                T: Into<JsValue>,
            {
                self.bindings.push(bind.into());
                self
            }

            pub async fn fetch<T>(&self) -> Result<Vec<T>, JsValue>
            where
                T: DeserializeOwned,
            {
                let promise = crate::exec_sql(&self.db, &self.query, self.bindings.clone().into_boxed_slice())
                    .dyn_into::<Promise>()?;
                let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
                let result: Vec<T> = from_value(result)?;
                Ok(result)
            }

            pub async fn execute(&self) -> Result<JsValue, JsValue> {
                let promise = crate::exec_sql(&self.db, &self.query, self.bindings.clone().into_boxed_slice())
                    .dyn_into::<Promise>()?;
                let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
                Ok(result)
            }
        }
    }
}
