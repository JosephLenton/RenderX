use crate::dom::Attribute;
use ::std::convert::Into;

#[derive(Clone, Debug)]
pub enum Node {
    Empty,
    Doctype {
    },
    Comment {
        pub(crate) children: Option<Vec<Node>>,
    },
    SelfClosing {
        pub(crate) name: &'static str,
        pub(crate) attributes: Option<Vec<Attribute>>,
    },
    OpenEmpty {
        pub(crate) name: &'static str,
        pub(crate) attributes: Option<Vec<Attribute>>,
    },
    OpenWithChildren {
        pub(crate) name: &'static str,
        pub(crate) attributes: Option<Vec<Attribute>>,
        pub(crate) contents: Vec<Node>,
    },
}

impl Node {
    pub fn new(
        name: &'static str,
        attributes: Option<Vec<Attribute>>,
        children: Option<Vec<Node>>,
    ) -> Self {
        Self {
            name,
            attributes,
            contents: children.into(),
        }
    }

    pub fn new_self_closing(name: &'static str, attributes: Option<Vec<Attribute>>) -> Self {
        Self {
            name,
            attributes,
            contents: Contents::SelfClosing,
        }
    }

    pub fn is_named(&self) -> bool {
        self.name != ""
    }

    /// Virtual nodes are nodes which don't have a name.
    /// i.e. `<></>` and `</>`.
    pub fn is_virtual(&self) -> bool {
        !self.is_named()
    }

    pub fn is_self_closing(&self) -> bool {
        if let Contents::SelfClosing = &self.contents {
            return true;
        }

        false
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Contents {
    SelfClosing,
    Empty,
    Some(Vec<Node>),
}

impl Into<Contents> for Vec<Node> {
    fn into(self) -> Contents {
        Contents::Some(self)
    }
}

impl Into<Contents> for Option<Vec<Node>> {
    fn into(self) -> Contents {
        match self {
            Some(children) => children.into(),
            None => Contents::Empty,
        }
    }
}
