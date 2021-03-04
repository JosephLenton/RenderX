use crate::dom::Attribute;
use crate::dom::Child;
use crate::dom::Node;
use ::std::convert::Into;
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
        match node {
            Node::Empty => {}
            Node::Doctype { name, attributes } => {
                write!(self.buffer, "<!{}", name)?;
                self.render_doctype_attributes(attributes)?;
                write!(self.buffer, ">")?;
            }
            Node::Comment { children } => match children {
                Some(children) => {
                    write!(self.buffer, "<!-- ")?;
                    self.render_nodes(children)?;
                    write!(self.buffer, " -->")?;
                }
                None => {
                    write!(self.buffer, "<!-- -->")?;
                }
            },
            Node::Fragment { children } => {
                self.render_nodes(children)?;
            }
            Node::SelfClosing { name, attributes } => {
                write!(self.buffer, "<{}", name)?;
                self.render_maybe_attributes(attributes)?;
                write!(self.buffer, "/>")?;
            }
            Node::OpenEmpty { name, attributes } => {
                write!(self.buffer, "<{}", name)?;
                self.render_maybe_attributes(attributes)?;
                write!(self.buffer, ">")?;
                write!(self.buffer, "</{}>", name)?;
            }
            Node::OpenWithChildren {
                name,
                attributes,
                child,
            } => {
                write!(self.buffer, "<{}", name)?;
                self.render_maybe_attributes(attributes)?;
                write!(self.buffer, ">")?;
                self.render_child(child)?;
                write!(self.buffer, "</{}>", name)?;
            }
            Node::Text { contents } => write!(self.buffer, "{}", contents)?,
        }

        Ok(())
    }

    fn render_doctype_attributes(&mut self, maybe_attributes: Option<Vec<Attribute>>) -> Result {
        match maybe_attributes {
            Some(attributes) => {
                for attribute in attributes {
                    self.render_doctype_attribute(attribute)?;
                }
            }
            None => {}
        }

        Ok(())
    }

    fn render_doctype_attribute(&mut self, attribute: Attribute) -> Result {
        write!(self.buffer, " {}", attribute.key)
    }

    fn render_maybe_attributes(&mut self, maybe_attributes: Option<Vec<Attribute>>) -> Result {
        match maybe_attributes {
            Some(attributes) => self.render_attributes(attributes),
            None => Ok(()),
        }
    }

    fn render_attributes(&mut self, attributes: Vec<Attribute>) -> Result {
        for attribute in attributes {
            match attribute.value {
                Some(text) => {
                    write!(self.buffer, " {}=\"{}\"", attribute.key, text)?;
                }
                None => {
                    write!(self.buffer, " {}", attribute.key)?;
                }
            }
        }

        Ok(())
    }

    fn render_maybe_nodes(&mut self, maybe_nodes: Option<Vec<Node>>) -> Result {
        match maybe_nodes {
            Some(nodes) => self.render_nodes(nodes),
            None => Ok(()),
        }
    }

    fn render_nodes(&mut self, nodes: Vec<Node>) -> Result {
        for node in nodes {
            self.render_node(node)?;
        }

        Ok(())
    }

    fn render_maybe_child(&mut self, maybe_child: Option<Child>) -> Result {
        match maybe_child {
            Some(child) => self.render_child(child),
            None => Ok(()),
        }
    }

    fn render_child(&mut self, child: Child) -> Result {
        match child {
            Child::None => Ok(()),
            Child::Nodes { nodes } => self.render_nodes(nodes),
            Child::Text { contents } => {
                write!(self.buffer, "{}", contents)?;
                Ok(())
            }
        }
    }
}

impl Into<String> for Render {
    fn into(self) -> String {
        self.buffer
    }
}
