use crate::component::ast::Function;
use crate::component::ast::Generics;
use crate::component::ast::Params;
use crate::component::ast::Public;

use ::proc_macro2::TokenStream;
use ::quote::quote;

pub fn build(ast: Function) -> TokenStream {
    visit_function(ast)
}

fn visit_function(f: Function) -> TokenStream {
    let public_tokens = visit_public(f.public);
    let name = f.name;
    let generics_tokens = visit_generics(f.generics);
    let params_tokens = visit_params(f.params);
    let rest = f.rest;

    quote! {
        #public_tokens fn #name #generics_tokens #params_tokens #rest
    }
}

fn visit_public(maybe_public: Option<Public>) -> TokenStream {
    quote! {}
}

fn visit_generics(maybe_generics: Option<Generics>) -> TokenStream {
    quote! {}
}

fn visit_params(params: Params) -> TokenStream {
    let params_tokens = params.tokens;

    quote! {
        #params_tokens
    }
}
