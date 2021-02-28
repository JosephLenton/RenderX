use ::proc_macro2::TokenStream;
use ::std::convert::From;
use ::std::fmt;

pub type Result<N> = ::std::result::Result<N, Error>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    EmptyMacroStreamGiven,
    UnexpectedStartingInput,
    UnexpectedToken,
    ExcessNodesFound,
    PeekOnEmptyNode,
    MoreTokensExpected,
    ChompOnEmptyNode,
    MismatchedTagName,
    FmtError(fmt::Error),
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FmtError(err)
    }
}
