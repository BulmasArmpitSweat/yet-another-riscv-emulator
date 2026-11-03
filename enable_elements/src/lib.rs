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

    let fields = data.fields
        .iter()
        .filter(|f| match &f.ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                path.is_ident("bool")
            },
            _ => false
        }
    ).collect::<Vec<_>>();

    let idents = fields
        .iter()
        .filter(|m| m.ident.is_some())
        .map(|m| m.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let mut running_string = String::new();
    running_string.push_str(format!("impl {} {{", name).as_str());

    for ident in idents {
        let ident_str = ident.to_string();
        let func_str = format!("fn enable_{}(mut self) -> Self {{ self.{} = true; self }}", ident_str, ident_str);
        running_string.push_str(func_str.as_str());
    }

    running_string.push('}');
    println!("{}", running_string.clone());

    running_string.parse().unwrap()
}