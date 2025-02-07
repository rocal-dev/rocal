use proc_macro2::{Span, TokenStream};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use syn::{spanned::Spanned, LitStr};

pub fn get_migrations(item: &TokenStream) -> Result<String, syn::Error> {
    let path_name: LitStr = if item.is_empty() {
        LitStr::new("db/migrations", item.span())
    } else {
        syn::parse2(item.clone())?
    };

    let path = resolve_path(&path_name.value(), item.span())?;

    let mut result = String::new();

    if let Ok(dir) = fs::read_dir(&path) {
        let mut entries: Vec<_> = dir.filter_map(Result::ok).collect();

        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();

            if path.is_file() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    result += &contents;
                } else {
                    return Err(syn::Error::new(
                        item.span(),
                        format!("{} cannot be opened", entry.file_name().to_str().unwrap()),
                    ));
                }
            }
        }

        Ok(result)
    } else {
        Err(syn::Error::new(
            item.span(),
            format!("{} not found", path_name.value()),
        ))
    }
}

fn resolve_path(path: impl AsRef<Path>, span: Span) -> syn::Result<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        return Err(syn::Error::new(
            span,
            "absolute paths will only work on the current machine",
        ));
    }

    if path.is_relative()
        && !path
            .parent()
            .map_or(false, |parent| !parent.as_os_str().is_empty())
    {
        return Err(syn::Error::new(
            span,
            "paths relative to the current file's directory are not currently supported",
        ));
    }

    let base_dir = env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        syn::Error::new(
            span,
            "CARGO_MANIFEST_DIR is not set; please use Cargo to build",
        )
    })?;

    let base_dir_path = Path::new(&base_dir);

    Ok(base_dir_path.join(path))
}
