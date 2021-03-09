use crate::component::ast::Function;
use crate::component::ast::Generics;
use crate::component::ast::Public;
use crate::component::ast::WhereClause;
use crate::component::error::Error;
use crate::component::error::Result;

use crate::util::TokenIterator;

use ::proc_macro2::token_stream::IntoIter;
use ::proc_macro2::Delimiter;
use ::proc_macro2::Group;
use ::proc_macro2::Ident;
use ::proc_macro2::TokenStream;
use ::std::fmt::Write;

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

    let mut input = TokenIterator::new(stream);
    let node = parse_function(&mut input)?;

    if !input.is_empty() {
        return Err(Error::ExcessNodesFound);
    }

    Ok(node)
}

fn parse_function(input: &mut TokenIteratorStream) -> Result<Function> {
    let public = parse_public(input)?;
    let name = parse_name(input)?;
    let generics = parse_generics(input)?;
    let params = parse_params(input)?;
    let where_clause = parse_where_clause(input)?;
    let code = parse_code(input)?;

    Ok(Function {
        public,
        name,
        generics,
        params,
        where_clause,
        code,
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

fn parse_params(input: &mut TokenIteratorStream) -> Result<Group> {
    Ok(input.chomp_group(Delimiter::Parenthesis)?)
}

fn parse_where_clause(input: &mut TokenIteratorStream) -> Result<Option<WhereClause>> {
    Ok(None)
}

fn parse_code(input: &mut TokenIteratorStream) -> Result<Group> {
    Ok(input.chomp_group(Delimiter::Brace)?)
}

// #[cfg(test)]
// mod parse {
//     use super::*;
//     use ::pretty_assertions::assert_eq;
//     use ::quote::quote;

//     fn assert_eq_nodes(tokens: TokenStream, expected_function: Function) -> Result<()> {
//         let nodes = parse(tokens.into())?;
//         assert_eq!(nodes, expected_nodes);

//         Ok(())
//     }
// }
