use crate::component::ast::Function;
use crate::component::ast::Props;

use ::proc_macro2::TokenStream;
use ::quote::quote;

pub fn build(ast: Function) -> TokenStream {
    visit_function(ast)
}

fn visit_function(f: Function) -> TokenStream {
    let visibility = f.visibility;
    let constness = f.constness;
    let asyncness = f.asyncness;
    let unsafety = f.unsafety;
    let name = f.name;
    let return_type = f.return_type;
    let code = f.code;

    let (args_patterns_tokens, args_types_tokens) = visit_args(f.props.as_ref());
    let props_type = visit_props_type(f.props.as_ref());

    quote! {
        #[allow(non_snake_case)]
        #visibility struct #name;

        impl FnOnce<#args_types_tokens> for #name
        {
            type Output = #return_type;
            #constness #asyncness #unsafety extern "rust-call" fn call_once(self, #args_patterns_tokens: #args_types_tokens) -> #return_type
                #code
        }

        impl ::renderx::Component for #name {
            type Props = #props_type;
        }
    }
}

fn visit_args(maybe_props: Option<&Props>) -> (TokenStream, TokenStream) {
    match maybe_props {
        None => (
            quote! {
                _
            },
            quote! {
                ()
            },
        ),
        Some(props) => {
            let attributes = &props.attributes;
            let name = &props.pattern;
            let item_type = &props.item_type;

            (
                quote! {
                    #(#attributes)* (#name,)
                },
                quote! {
                    (#item_type,)
                },
            )
        }
    }
}

fn visit_props_type(maybe_props: Option<&Props>) -> TokenStream {
    match maybe_props {
        None => {
            quote! {
                ()
            }
        }
        Some(props) => {
            let item_type = &props.item_type;

            quote! {
                #item_type
            }
        }
    }
}
