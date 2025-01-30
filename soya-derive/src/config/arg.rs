use quote::ToTokens;
use syn::Path;

use super::ArgParser;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Argument {
    Policy,
}

impl ArgParser for Argument {
    fn parse(input: &mut syn::parse::ParseStream) -> syn::Result<(Self, bool)> {
        let path: Path = input.parse()?;

        if let Some(ident) = path.get_ident() {
            let arg = ident.to_string();

            Ok(match arg.as_str() {
                "policy" => (Self::Policy, true),
                _ => {
                    unimplemented!()
                }
            })
        } else {
            // let method = path.to_token_stream().to_string();
            // let method = method.replace(char::is_whitespace, "");

            // Ok((Self::MethodCall(method), true))
            unimplemented!()
        }
    }
}
