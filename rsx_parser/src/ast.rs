#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ASTNode {
    tag: &'static str,
    is_self_closing: bool,
    // attrs: Vec<ASTAttribute>,
    // children: Vec<ASTChild>,
}

impl ASTNode {
    pub fn new(tag: &'static str, is_self_closing: bool) -> Self {
        Self {
            tag,
            is_self_closing,
        }
    }
}

/*
#[derive(Copy, Clone, Debug)]
pub enum ASTChild {
  RawText(&'a str),
  Node(Node<'a>)
}

#[derive(Copy, Clone, Debug)]
pub enum ASTAttribute<'a> {
  Pair(&'a str, &'a str),
  Single(&'a str),
}
*/
