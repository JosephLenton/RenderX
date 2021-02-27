#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Tag(Tag),
    Literal(Literal),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Text(String),
    Code(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tag {
    pub tag: String,
    pub is_self_closing: bool,
    pub attributes: Option<Vec<Attribute>>,
    pub children: Option<Vec<Node>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: Option<Literal>,
}
