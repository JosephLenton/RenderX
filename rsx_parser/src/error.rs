use ::std::convert::From;
use ::std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    EmptyMacroStreamGiven,
    UnexpectedInput,
    UnexpectedToken,
    ExcessNodesFound,
    PeekOnEmptyNode,
    ChompOnEmptyNode,
    MismatchedTagName,
    FmtError(fmt::Error),
}

pub type Result<N> = ::std::result::Result<N, Error>;

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FmtError(err)
    }
}
