#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Empty,
    Doctype {
        name: String,
        attributes: Option<Vec<Attribute>>,
    },
    Comment {
        children: Option<Vec<Node>>,
    },
    Fragment {
        children: Vec<Node>,
    },
    /// Self closing tags. i.e. <hr />
    SelfClosing {
        name: String,
        attributes: Option<Vec<Attribute>>,
    },
    /// Tags that have children. i.e. <div></div>
    Open {
        name: String,
        attributes: Option<Vec<Attribute>>,
        children: Option<Vec<Node>>,
    },
    Text(String),
    Code(String),
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
