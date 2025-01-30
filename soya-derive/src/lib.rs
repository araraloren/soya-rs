mod config;
mod gen;
mod value;

use gen::SoyaGenerator;
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::DataEnum;
use syn::DeriveInput;

fn error(spanned: impl Spanned, msg: impl Into<String>) -> syn::Error {
    syn::Error::new(spanned.span(), msg.into())
}

#[proc_macro_derive(Soya, attributes(soya, arg, pos, cmd, sub))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let generator = |input: DeriveInput| -> syn::Result<proc_macro2::TokenStream> {
        let mut cg = SoyaGenerator::new(&input)?;

        cg.generate_impl()
    };

    let ts = generator(input).unwrap_or_else(syn::Error::into_compile_error);

    quote! {
        #ts
    }
    .into()
}
