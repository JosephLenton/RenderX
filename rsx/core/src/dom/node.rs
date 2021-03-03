use crate::dom::Attribute;
use crate::dom::Child;
use crate::dom::ToChild;
use ::std::convert::Into;

#[derive(Clone, Debug)]
pub enum Node {
    Empty,
    Doctype {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
    },
    Comment {
        children: Option<Vec<Self>>,
    },
    Fragment {
        children: Vec<Self>,
    },
    SelfClosing {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
    },
    OpenEmpty {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
    },
    OpenWithChildren {
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
        child: Child,
    },
    Text {
        contents: &'static str,
    },
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
