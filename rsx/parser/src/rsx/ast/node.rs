use crate::rsx::ast::Attribute;
use crate::rsx::ast::Value;
use crate::util::token_stream_eq;
use ::proc_macro2::TokenStream;

#[derive(Clone, Debug)]
pub enum Node {
    Empty,
    Doctype {
        name: Value,
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
        name: Value,
        attributes: Option<Vec<Attribute>>,
    },
    /// Tags that have children. i.e. <div></div>
    Open {
        name: Value,
        attributes: Option<Vec<Attribute>>,
        children: Option<Vec<Node>>,
    },
    Text(String),
    Code(TokenStream),
}

/// PartialEq is needed for assert_eq.
/// TokenStream doesn't support PartialEq.
///
/// Instead we do a debug string comparison of TokenStream,
/// and we only support this for tests.
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Empty, Node::Empty) => true,
            (
                Node::Doctype {
                    name: left_name,
                    attributes: left_attributes,
                },
                Node::Doctype {
                    name: right_name,
                    attributes: right_attributes,
                },
            ) => left_name == right_name && left_attributes == right_attributes,
            (
                Node::Comment {
                    children: left_children,
                },
                Node::Comment {
                    children: right_children,
                },
            ) => left_children == right_children,
            (
                Node::Fragment {
                    children: left_children,
                },
                Node::Fragment {
                    children: right_children,
                },
            ) => left_children == right_children,
            (
                Node::SelfClosing {
                    name: left_name,
                    attributes: left_attributes,
                },
                Node::SelfClosing {
                    name: right_name,
                    attributes: right_attributes,
                },
            ) => left_name == right_name && left_attributes == right_attributes,
            (
                Node::Open {
                    name: left_name,
                    attributes: left_attributes,
                    children: left_children,
                },
                Node::Open {
                    name: right_name,
                    attributes: right_attributes,
                    children: right_children,
                },
            ) => {
                left_name == right_name
                    && left_attributes == right_attributes
                    && left_children == right_children
            }
            (Node::Text(left), Node::Text(right)) => left == right,
            (Node::Code(left), Node::Code(right)) => token_stream_eq(&left, &right),
            _ => false,
        }
    }
}
