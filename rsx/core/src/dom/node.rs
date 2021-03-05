use crate::dom::Attribute;
use crate::dom::Child;
use crate::dom::ToChild;
use ::std::convert::Into;

/// The contents of the Node are all doc-hidden.
/// This is because the Node structure may change in future releases.
///
/// You can look in the source code if you want to use it directly.
/// Just be aware a future release might break your code.
#[derive(Clone, Debug)]
pub enum Node {
    #[doc(hidden)]
    Empty,

    #[doc(hidden)]
    Doctype {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
    },

    #[doc(hidden)]
    Comment { children: Option<Vec<Self>> },

    #[doc(hidden)]
    Fragment { children: Vec<Self> },

    #[doc(hidden)]
    SelfClosing {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
    },

    #[doc(hidden)]
    OpenEmpty {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
    },

    #[doc(hidden)]
    OpenWithChildren {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
        child: Child,
    },

    #[doc(hidden)]
    Text { contents: &'static str },
}

impl Node {
    pub fn new_open<N>(
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
        maybe_child: Option<N>,
    ) -> Self
    where
        N: ToChild,
    {
        match maybe_child {
            Some(child) => Self::OpenWithChildren {
                name,
                attributes,
                child: child.to_child(),
            },
            None => Self::OpenEmpty { name, attributes },
        }
    }

    pub fn new_self_closing(name: &'static str, attributes: Option<Vec<Attribute>>) -> Self {
        Self::SelfClosing { name, attributes }
    }
}

#[cfg(test)]
mod node {
    use super::*;

    #[test]
    fn it_should_build_a_node() {
        let text = "yo";

        let node = crate::dom::Node::new_open(
            "h1",
            None,
            Some(vec![
                crate::dom::ToNode::to_node("Hello world!"),
                crate::dom::ToNode::to_node(crate::dom::Node::Text {
                    contents: "hello world!",
                }),
                crate::dom::ToNode::to_node(text),
            ]),
        );

        if let Node::OpenWithChildren {
            name,
            attributes,
            child,
        } = node
        {
            assert_eq!(name, "h1");
        } else {
            unreachable!();
        }
    }
}
