#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ASTError {
    UnexpectedInput,
    UnexpectedToken,
    ExcessNodesFound,
    PeekOnEmptyNonde,
    ChompOnEmptyNonde,
    MismatchedTagName,
}
