use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

use crate::ast::Attribute;
use crate::ast::Literal;
use crate::ast::Node;
use crate::ast::Tag;
use crate::error::Result;

// static BUFFER_NAME: &'static str = "__";

pub fn build(ast: Node) -> Result<TokenStream> {
    // let buffer_name = format_ident!("{}", BUFFER_NAME);
    Ok(quote! {})
}
