use crate::component::ast::Function;
use crate::component::ast::Generics;
use crate::component::ast::Params;
use crate::component::ast::Public;
use crate::component::error::Error;
use crate::component::error::Result;

use crate::util::TokenIterator;

use ::proc_macro2::token_stream::IntoIter;
use ::proc_macro2::Delimiter;
use ::proc_macro2::Ident;
use ::proc_macro2::TokenStream;

const COLON: char = ':';
const EXCLAMATION_MARK: char = '!';
const HYPHEN: char = '-';
const LEFT_ANGLE: char = '<';
const RIGHT_ANGLE: char = '>';
const FORWARD_SLASH: char = '/';
const EQUALS: char = '=';

type TokenIteratorStream = TokenIterator<IntoIter>;

pub fn parse(stream: TokenStream) -> Result<Function> {
    if stream.is_empty() {
        return Err(Error::EmptyMacroStreamGiven);
    }

    let input = TokenIterator::new(stream);
    let node = parse_function(input)?;

    Ok(node)
}

fn parse_function(mut input: TokenIteratorStream) -> Result<Function> {
    input.chomp_ident_of("fn")?;

    let public = parse_public(&mut input)?;
    let name = parse_name(&mut input)?;
    let generics = parse_generics(&mut input)?;
    let params = parse_params(&mut input)?;
    let rest = parse_rest(input)?;

    Ok(Function {
        public,
        name,
        generics,
        params,
        rest,
    })
}

fn parse_public(input: &mut TokenIteratorStream) -> Result<Option<Public>> {
    Ok(None)
}

fn parse_name(input: &mut TokenIteratorStream) -> Result<Ident> {
    Ok(input.chomp_ident()?)
}

fn parse_generics(input: &mut TokenIteratorStream) -> Result<Option<Generics>> {
    Ok(None)
}

fn parse_params(input: &mut TokenIteratorStream) -> Result<Params> {
    let tokens = input.chomp_group(Delimiter::Parenthesis)?;

    Ok(Params { tokens })
}

fn parse_rest(mut input: TokenIteratorStream) -> Result<TokenStream> {
    if input.is_empty() {
        return Err(Error::ExpectRestTokens);
    }

    Ok(input.to_token_stream())
}
