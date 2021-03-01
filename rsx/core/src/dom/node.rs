use crate::dom::Attribute;
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
        children: Vec<Self>,
    },
    Text {
        contents: &'static str,
    },
}

impl Node {
    pub fn new_open(
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
        maybe_children: Option<Vec<Self>>,
    ) -> Self {
        match maybe_children {
            Some(children) => Self::OpenWithChildren {
                name,
                attributes,
                children,
            },
            None => Self::OpenEmpty { name, attributes },
        }
    }

    pub fn new_self_closing(name: &'static str, attributes: Option<Vec<Attribute>>) -> Self {
        Self::SelfClosing { name, attributes }
    }
}
