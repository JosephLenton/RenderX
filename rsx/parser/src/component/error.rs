use ::std::convert::From;
use ::std::fmt;

use crate::util::TokenIteratorError;

pub type Result<N> = ::std::result::Result<N, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NoReturnType,
    ExtraParametersFound,
    SelfArgUnsupported,
    AttributeFound,
    ExpectRestTokens,
    EmptyMacroStreamGiven,
    ExcessTokensFound,
    UnexpectedToken,
    ChompOnEmptyNode,
    InternalPropsArgParsingMismatchError,
    SynError(syn::parse::Error),
    FmtError(fmt::Error),
}

impl From<syn::parse::Error> for Error {
    fn from(err: syn::parse::Error) -> Self {
        Error::SynError(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FmtError(err)
    }
}

impl From<TokenIteratorError> for Error {
    fn from(err: TokenIteratorError) -> Self {
        match err {
            TokenIteratorError::ChompOnEmptyNode => Error::ChompOnEmptyNode,
            TokenIteratorError::UnexpectedToken => Error::UnexpectedToken,
        }
    }
}
