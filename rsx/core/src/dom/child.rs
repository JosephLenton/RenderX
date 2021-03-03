use crate::dom::Node;

#[derive(Clone, Debug)]
pub enum Child {
    None,
    Nodes { nodes: Vec<Node> },
    Text { contents: &'static str },
}
