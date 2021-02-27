use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

mod ast;
mod error;
mod grammar;
mod output;

use crate::error::Result;

pub fn parse(old_stream: TokenStream) -> Result<TokenStream> {
    let stream = TokenStream::from(old_stream);

    let ast = grammar::parse(stream)?;
    output::build(ast)
}
