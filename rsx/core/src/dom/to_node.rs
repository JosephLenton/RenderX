use crate::dom::Node;

pub trait ToNode {
    fn to_node(self) -> Node;
}

impl ToNode for Node {
    fn to_node(self) -> Self {
        self
    }
}

impl<N: ToNode> ToNode for Option<N> {
    fn to_node(self) -> Node {
        match self {
            None => Node::Empty,
            Some(n) => n.to_node(),
        }
    }
}

impl ToNode for &&'static str {
    fn to_node(self) -> Node {
        Node::Text { contents: *self }
    }
}

impl ToNode for &'static str {
    fn to_node(self) -> Node {
        Node::Text { contents: self }
    }
}

impl ToNode for Vec<&'static str> {
    fn to_node(self) -> Node {
        if self.len() == 0 {
            Node::Empty
        } else if self.len() == 1 {
            Node::Text { contents: self[0] }
        } else {
            Node::Fragment {
                children: self
                    .into_iter()
                    .map(|text| Node::Text { contents: text })
                    .collect(),
            }
        }
    }
}

impl ToNode for Vec<Node> {
    fn to_node(self) -> Node {
        Node::Fragment { children: self }
    }
}

impl ToNode for () {
    fn to_node(self) -> Node {
        Node::Empty
    }
}
