#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ASTError {
    UnexpectedInput,
    UnexpectedToken,
    PeekOnEmptyNonde,
    ChompOnEmptyNonde,
}
