use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn load_migrations(input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(input as LitStr);
    let rel_path = path_lit.value();

    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(v) => v,
        Err(_) => {
            return syn::Error::new_spanned(
                path_lit,
                "CARGO_MANIFEST_DIR is not set (this is a compiler bug)",
            )
            .to_compile_error()
            .into();
        }
    };

    let path = std::path::Path::new(&manifest_dir).join(&rel_path);

    if !path.exists() {
        return syn::Error::new_spanned(
            path_lit,
            format!(
                "directory '{}' does not exist (resolved to '{}')",
                rel_path,
                path.display()
            ),
        )
        .to_compile_error()
        .into();
    }

    // Читаем .sql файлы
    let mut entries: Vec<_> = match std::fs::read_dir(&path) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("sql"))
            .collect(),
        Err(err) => {
            return syn::Error::new_spanned(
                path_lit,
                format!("failed to read directory '{}': {err}", path.display()),
            )
            .to_compile_error()
            .into();
        }
    };

    // Детерминированный порядок
    entries.sort();

    let mut ids = Vec::new();
    let mut contents = Vec::new();

    for file in entries {
        let name = match file.file_stem().and_then(|s| s.to_str()) {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => continue,
        };

        let sql = match std::fs::read_to_string(&file) {
            Ok(c) if !c.is_empty() => c,
            _ => continue,
        };

        ids.push(name);
        contents.push(sql);
    }

    // Генерация Rust-кода
    let expanded = quote! {{
        let ids: ::std::vec::Vec<::std::string::String> = vec![
            #( ::std::string::String::from(#ids) ),*
        ];

        let queries: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String
        > = {
            let mut m = ::std::collections::HashMap::new();
            #(
                m.insert(
                    ::std::string::String::from(#ids),
                    ::std::string::String::from(#contents),
                );
            )*
            m
        };

        (ids, queries)
    }};

    expanded.into()
}
