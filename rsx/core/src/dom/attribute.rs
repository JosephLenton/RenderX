#[derive(Clone, Debug)]
pub struct Attribute {
    pub(crate) key: &'static str,
}

impl Attribute {
    pub fn new(key: &'static str) -> Self {
        Self { key }
    }
}
