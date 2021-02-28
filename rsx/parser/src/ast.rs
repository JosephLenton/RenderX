#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub name: String,
    pub is_self_closing: bool,
    pub attributes: Option<Vec<Attribute>>,
    pub children: Option<Vec<Child>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Child {
    Node(Node),
    Text(String),
    Code(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: Option<AttributeValue>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AttributeValue {
    Text(String),
    Code(String),
}
