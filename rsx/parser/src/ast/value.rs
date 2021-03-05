use crate::token_stream_compare::token_stream_eq;
use ::proc_macro2::TokenStream;

#[derive(Clone, Debug)]
pub enum Value {
    Text(String),
    Code(TokenStream),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Text(left), Value::Text(right)) => left == right,
            (Value::Code(left), Value::Code(right)) => token_stream_eq(&left, &right),
            _ => false,
        }
    }
}
