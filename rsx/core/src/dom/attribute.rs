#[derive(Clone, Debug)]
pub struct Attribute {
    pub key: &'static str,
    pub value: Option<&'static str>,
}

impl Attribute {
    pub fn new(key: &'static str, value: Option<&'static str>) -> Self {
        Self { key, value }
    }
}
