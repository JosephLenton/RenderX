use crate::dom::Node;

#[derive(Clone, Debug)]
pub enum Child {
    Text(&'static str),
    Node(Node),
}
