use crate::ast::Value;
use crate::token_stream_compare::token_stream_eq;
use ::proc_macro2::TokenStream;

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