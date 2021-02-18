use crate::Render;
use ::std::convert::From;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::fmt::Write;

#[derive(Clone, Debug)]
pub struct ServerRender {
    buffer: String,
}

impl ServerRender {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

impl Render for ServerRender {
    fn render<D: Display>(&mut self, item: &D) -> Result {
        write!(self.buffer, "{}", item)
    }
}

impl Display for ServerRender {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.buffer)
    }
}

impl<'a> From<&'a ServerRender> for &'a str {
    fn from(render: &'a ServerRender) -> Self {
        &render.buffer
    }
}

impl From<ServerRender> for String {
    fn from(render: ServerRender) -> Self {
        render.buffer.clone()
    }
}
