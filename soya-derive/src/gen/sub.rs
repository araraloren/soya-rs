use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub struct SubGenerator<'a> {
    ident: &'a Ident,
}

impl<'a> SubGenerator<'a> {
    pub fn new(input: &'a DeriveInput) -> syn::Result<Self> {
        let ident = &input.ident;

        Ok(Self { ident })
    }

    pub fn generate_impl(&mut self) -> syn::Result<TokenStream> {
        Ok(quote! {})
    }
}
