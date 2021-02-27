#[derive(Debug)]
pub struct Node {
    tag: &'static str
    is_self_closing: bool
    attrs: Option<Vec<Attribute>>
    children: Option<Vec<ChildNode>>
}

#[derive(Debug)]
pub enum Child {
    StaticText(&'static str)
}

#[derive(Debug)]
pub struct Attribute {
    key: &'static str
}
