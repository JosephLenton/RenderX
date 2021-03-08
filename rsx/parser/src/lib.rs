use ::proc_macro2::TokenStream;

mod ast;
pub mod error;
mod grammar;
mod output;
pub(crate) mod token_stream;

use crate::error::Result;

pub fn parse(old_stream: TokenStream) -> Result<TokenStream> {
    let stream = TokenStream::from(old_stream);

    let ast = grammar::parse(stream)?;
    Ok(output::build(ast))
}
