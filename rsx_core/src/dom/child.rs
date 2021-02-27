#[derive(Clone, Debug)]
pub enum Child {
    StaticText(&'static str),
}
