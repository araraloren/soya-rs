use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Generics};

use crate::config::soya::Argument;
use crate::config::Configs;

pub struct SoyaGenerator<'a> {
    name: TokenStream,

    ident: &'a Ident,

    generics: &'a Generics,

    configs: Configs<Argument>,
}

impl<'a> SoyaGenerator<'a> {
    pub fn new(input: &'a DeriveInput) -> syn::Result<Self> {
        let ident = &input.ident;
        let generics = &input.generics;
        let configs = Configs::<Argument>::parse_attrs("soya", &input.attrs);
        let name = quote! {};

        Ok(Self {
            ident,
            generics,
            configs,
            name,
        })
    }

    pub fn generate_impl(&mut self) -> syn::Result<TokenStream> {
        Ok(quote! {})
    }
}
