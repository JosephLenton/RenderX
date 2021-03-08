use crate::ast::Value;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub key: Value,
    pub value: Option<Value>,
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}
