use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn naughty_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let key = to_snake_case(&ident.to_string());
    let (impl_g, ty_g, where_c) = input.generics.split_for_impl();

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #input

        impl #impl_g crate::naughty_attribute::INaughtyAttributeMeta for #ident #ty_g #where_c {
            const KEY: &'static str = #key;
        }
    }
    .into()
}

fn to_snake_case(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.char_indices() {
        if ch.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}
