use find_crate::{find_crate, Manifest};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[allow(dead_code)]

/// TODO move to hello_macro
fn import() -> TokenStream {
    let manifest = Manifest::new().unwrap();

    match manifest.find(|s| s == "find-crate") {
        Some(package) => {
            let name = Ident::new(&package.name, Span::call_site());
            // If your proc-macro crate is 2018 edition, use `quote!(use #name as _foo;)` instead.
            // quote!(extern crate #name as _foo;)
            quote!(use #name as _foo;)
        }
        None => {
            quote!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import() {
        let _version = env!("CARGO_PKG_VERSION");
        let key: Option<&'static str> = option_env!("CARGO_MANIFEST_DIR");
        println!("the secret key might be: {:?}", key);
        let trace: Option<&'static str> = option_env!("RUST_BACKTRACE");
        println!("the secret key might be: {:?}", trace);

        let ts = import();
        let tss = ts.to_string();
        println!("crate name from TokenStream: {}", tss);
    }
}
