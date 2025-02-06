use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Token,
};

pub fn build_config_struct() -> TokenStream {
    quote! {
        pub struct Configuration {
            app_id: String,
            sync_server_endpoint: String,
            database: std::sync::Arc<crate::Database>,
        }

        impl Configuration {
            pub fn new(app_id: String, sync_server_endpoint: String, database: std::sync::Arc<crate::Database>) -> Self {
                Configuration {
                    app_id,
                    sync_server_endpoint,
                    database,
                }
            }

            pub fn get_app_id(&self) -> &str {
                &self.app_id
            }

            pub fn get_sync_server_endpoint(&self) -> &str {
                &self.sync_server_endpoint
            }

            pub fn get_database(&self) -> std::sync::Arc<crate::Database> {
                self.database.clone()
            }
        }
    }
}

pub fn parse_config(item: TokenStream) -> Result<ParsedConfig, syn::Error> {
    let parsed_config: ParsedConfig = syn::parse(item.into())?;

    Ok(parsed_config)
}

#[derive(Debug, Default)]
pub struct ParsedConfig {
    app_id: Option<String>,
    sync_server_endpoint: Option<String>,
    database_directory_name: Option<String>,
    database_file_name: Option<String>,
}

impl ParsedConfig {
    pub fn set_app_id(&mut self, app_id: String) {
        self.app_id = Some(app_id);
    }

    pub fn set_sync_server_endpoint(&mut self, endpoint: String) {
        self.sync_server_endpoint = Some(endpoint);
    }

    pub fn set_database_directory_name(&mut self, directory_name: String) {
        self.database_directory_name = Some(directory_name);
    }

    pub fn set_database_file_name(&mut self, file_name: String) {
        self.database_file_name = Some(file_name);
    }

    pub fn get_app_id(&self) -> &Option<String> {
        &self.app_id
    }

    pub fn get_sync_server_endpoint(&self) -> &Option<String> {
        &self.sync_server_endpoint
    }

    pub fn get_database_directory_name(&self) -> &Option<String> {
        &self.database_directory_name
    }

    pub fn get_database_file_name(&self) -> &Option<String> {
        &self.database_file_name
    }
}

impl Parse for ParsedConfig {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut config = ParsedConfig::default();

        let kvs = Punctuated::<KeyValue, Token!(,)>::parse_terminated(&input)?;
        let mut has_error_attribute = false;

        kvs.into_iter().for_each(|kv| match kv.key.as_str() {
            "app_id" => config.set_app_id(kv.value),
            "sync_server_endpoint" => config.set_sync_server_endpoint(kv.value),
            "database_directory_name" => config.set_database_directory_name(kv.value),
            "database_file_name" => config.set_database_file_name(kv.value),
            _ => has_error_attribute = true,
        });

        if has_error_attribute {
            return Err(syn::Error::new(
                input.span(),
                "You put (an) invalid attribute(s)",
            ));
        }

        Ok(config)
    }
}

struct KeyValue {
    key: String,
    value: String,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let key = input
            .parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| syn::Error::new(input.span(), "should have property keys"))?;

        let _: Token!(:) = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "prop name and value should be separated by :")
        })?;

        let value = input
            .parse()
            .map(|v: LitStr| v.value())
            .map_err(|_| syn::Error::new(input.span(), "Value should be here"))?;

        Ok(KeyValue { key, value })
    }
}
