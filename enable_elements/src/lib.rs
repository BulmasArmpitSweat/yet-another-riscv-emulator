extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(EnableElement)]
pub fn enable_element(input: TokenStream) -> TokenStream {
    let input_tokens = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input_tokens.ident;

    let data = match input_tokens.data {
        syn::Data::Enum(_) => panic!("Cannot operate on enums. Only structs are supported"),
        syn::Data::Union(_) => panic!("Cannot operate on unions. Only structs are supported"),
        syn::Data::Struct(s) => s
    };

    let members = data.fields
        .members()
        .into_iter()
        .filter(|m| {
            match m {
                syn::Member::Named(_) => true,
                _ => false
            }
        })
        .map(|m| {
            match m {
                syn::Member::Named(n) => n,
                _ => unreachable!()
            }
        })
        .collect::<Vec<_>>();

    let mut running_string = String::new();
    running_string.push_str(format!("impl {} {{", name).as_str());

    for member in members {
        let func_str = format!("fn enable_{}(mut self) -> Self {{ self.{} = true; self }}", member.to_string(), member.to_string());
        running_string.push_str(func_str.as_str());
    }

    running_string.push('}');

    running_string.parse().unwrap()
}