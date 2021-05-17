extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derives;

#[proc_macro_derive(DeriveEntity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, attrs, ..
    } = parse_macro_input!(input);

    match derives::expend_derive_entity(ident, attrs) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(DerivePrimaryKey)]
pub fn derive_primary_key(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, ..
    } = parse_macro_input!(input);

    match derives::expend_derive_primary_key(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(DeriveColumn)]
pub fn derive_column(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, ..
    } = parse_macro_input!(input);

    match derives::expend_derive_column(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(DeriveModel)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, ..
    } = parse_macro_input!(input);

    match derives::expend_derive_model(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(FromQueryResult)]
pub fn derive_from_query_result(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, ..
    } = parse_macro_input!(input);

    match derives::expend_derive_from_query_result(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}