use crate::dom::AttributeValue;

#[derive(Clone, Debug)]
pub struct Attribute {
    #[doc(hidden)]
    pub key: &'static str,

    #[doc(hidden)]
    pub value: AttributeValue,
}

impl Attribute {
    pub fn new(key: &'static str, value: AttributeValue) -> Self {
        Self { key, value }
    }
}
