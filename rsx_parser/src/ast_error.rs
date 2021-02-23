use ::std::convert::From;
use ::std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ASTError {
    UnexpectedInput,
    UnexpectedToken,
    ExcessNodesFound,
    PeekOnEmptyNode,
    ChompOnEmptyNode,
    MismatchedTagName,
    FmtError(fmt::Error),
}

pub type Result<N> = ::std::result::Result<N, ASTError>;

impl From<fmt::Error> for ASTError {
    fn from(err: fmt::Error) -> Self {
        ASTError::FmtError(err)
    }
}

// impl Into<Result<()>> for fmt::Result {
//     fn into(self) -> Result<T> {
//         match self {
//             Err(error) => Err(error),
//             Ok(ok) => Ok(ok),
//         }
//     }
// }
