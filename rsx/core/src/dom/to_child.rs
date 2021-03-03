use crate::dom::Child;
use crate::dom::Node;

pub trait ToChild {
    fn to_child(self) -> Child;
}

impl<N: ToChild> ToChild for Option<N> {
    fn to_child(self) -> Child {
        match self {
            None => Child::None,
            Some(n) => n.to_child(),
        }
    }
}

impl ToChild for &'static str {
    fn to_child(self) -> Child {
        Child::Text { contents: self }
    }
}

impl ToChild for Vec<Node> {
    fn to_child(self) -> Child {
        Child::Nodes { nodes: self }
    }
}

impl ToChild for Node {
    fn to_child(self) -> Child {
        match self {
            Node::Empty => Child::None,
            node => Child::Nodes { nodes: vec![node] },
        }
    }
}

impl ToChild for () {
    fn to_child(self) -> Child {
        Child::None
    }
}
