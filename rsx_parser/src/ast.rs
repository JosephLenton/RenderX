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
    Literal(Literal),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Text(String),
    Code(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: Option<Literal>,
}
