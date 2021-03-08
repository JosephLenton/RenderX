use ::proc_macro2::TokenStream;

mod ast;
mod error;
mod grammar;
mod output;
pub(crate) mod token_stream;

pub use self::error::*;

pub fn parse(old_stream: TokenStream) -> Result<TokenStream> {
    let stream = TokenStream::from(old_stream);

    let ast = grammar::parse(stream)?;
    Ok(output::build(ast))
}
