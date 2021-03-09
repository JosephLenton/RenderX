use ::std::convert::From;
use ::std::fmt;

use crate::util::TokenIteratorError;

pub type Result<N> = ::std::result::Result<N, Error>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    AttributeFound,
    // ExpectedName,
    EmptyMacroStreamGiven,
    // UnexpectedStartingInput,
    ExcessNodesFound,
    // PeekOnEmptyNode,
    // MoreTokensExpected,
    UnexpectedToken,
    ChompOnEmptyNode,
    FmtError(fmt::Error),
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
