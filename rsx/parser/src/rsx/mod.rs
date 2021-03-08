mod ast;
mod error;
mod grammar;
mod output;

pub use self::error::*;

use ::proc_macro2::TokenStream;

pub fn parse(old_stream: TokenStream) -> Result<TokenStream> {
    let stream = TokenStream::from(old_stream);
    let ast = grammar::parse(stream)?;
    Ok(output::build(ast))
}
