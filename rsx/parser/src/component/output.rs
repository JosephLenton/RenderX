use crate::component::ast::Function;
use crate::component::ast::Generics;
use crate::component::ast::Public;
use crate::component::ast::WhereClause;

use ::proc_macro2::TokenStream;
use ::quote::quote;

pub fn build(ast: Function) -> TokenStream {
    visit_function(ast)
}

fn visit_function(f: Function) -> TokenStream {
    let public_tokens = visit_public(f.public);
    let name = f.name;
    let generics_tokens = visit_generics(f.generics);
    let params = f.params;
    let where_clause_tokens = visit_where_clause(f.where_clause);
    let code = f.code;

    quote! {
        #public_tokens #name #generics_tokens #params #where_clause_tokens #code
    }
}

fn visit_public(maybe_public: Option<Public>) -> TokenStream {
    quote! {}
}

fn visit_generics(maybe_generics: Option<Generics>) -> TokenStream {
    quote! {}
}

fn visit_where_clause(maybe_where_clause: Option<WhereClause>) -> TokenStream {
    quote! {}
}
