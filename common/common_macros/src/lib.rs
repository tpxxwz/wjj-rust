use proc_macro::TokenStream;
use quote::quote;
use syn::{ExprTuple, parse_macro_input};

#[proc_macro]
pub fn register_template(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as ExprTuple);

    let name = match &args.elems[0] {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(s) => s.clone(),
            _ => panic!("template name must be a string literal"),
        },
        _ => panic!("template name must be a string literal"),
    };

    let content = match &args.elems[1] {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(s) => s.clone(),
            _ => panic!("template content must be a string literal"),
        },
        _ => panic!("template content must be a string literal"),
    };

    let expanded = quote! {
        inventory::submit! {
            my_template_core::TemplateRegistration {
                f: |env| {
                    env.add_template(#name, #content).unwrap();
                }
            }
        }
    };

    expanded.into()
}
