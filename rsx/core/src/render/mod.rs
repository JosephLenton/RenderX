use crate::dom::Attribute;
use crate::dom::Child;
use crate::dom::Contents;
use crate::dom::Node;
use ::std::convert::Into;
use ::std::fmt::Display;
use ::std::fmt::Result;
use ::std::fmt::Write;

pub fn render(node: Node) -> String {
    let mut render = Render::new();
    render.render(node);
    render.into()
}

#[derive(Clone, Debug)]
pub struct Render {
    buffer: String,
}

impl Render {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn render(&mut self, node: Node) -> Result {
        self.render_node(node)
    }

    fn render_node(&mut self, node: Node) -> Result {
        let is_named = node.is_named();

        if is_named {
            write!(self.buffer, "<{}", node.name)?;
        }

        self.render_maybe_attributes(node.attributes)?;

        match node.contents {
            Contents::SelfClosing => {
                if is_named {
                    write!(self.buffer, "/>")?;
                }
            }
            Contents::Empty => {
                if is_named {
                    write!(self.buffer, "></{}>", node.name)?;
                }
            }
            Contents::Some(children) => {
                write!(self.buffer, ">")?;
                self.render_children(children)?;

                if is_named {
                    write!(self.buffer, "</{}>", node.name)?;
                }
            }
        }

        Ok(())
    }

    fn render_maybe_attributes(&mut self, maybe_attributes: Option<Vec<Attribute>>) -> Result {
        match maybe_attributes {
            Some(attributes) => self.render_attributes(attributes),
            None => Ok(()),
        }
    }

    fn render_attributes(&mut self, attributes: Vec<Attribute>) -> Result {
        for attribute in attributes {
            write!(self.buffer, " {}", attribute.key)?;
        }

        Ok(())
    }

    fn render_children(&mut self, children: Vec<Child>) -> Result {
        for child in children {
            self.render_child(child)?;
        }

        Ok(())
    }

    fn render_child(&mut self, child: Child) -> Result {
        match child {
            Child::Text(text) => write!(self.buffer, "{}", text),
            Child::Node(node) => self.render_node(node),
        }
    }
}

impl Into<String> for Render {
    fn into(self) -> String {
        self.buffer
    }
}
