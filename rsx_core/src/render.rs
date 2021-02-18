use ::std::fmt::Display;
use ::std::fmt::Result;

pub trait Render {
    fn render<D: Display>(&mut self, item: &D) -> Result;
}
