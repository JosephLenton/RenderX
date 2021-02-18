use crate::ASTError;
use crate::TokenIterator;
use ::proc_macro2::Ident;
use ::proc_macro2::Punct;
use ::proc_macro2::Spacing;
use ::proc_macro2::Span;
use ::proc_macro2::TokenStream;

pub type Result<N> = ::std::result::Result<N, ASTError>;

#[derive(Clone, PartialEq, Debug)]
pub enum Root {
    Node(Node),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    tag: String,
    is_self_closing: bool,
    // attrs: Vec<ASTAttribute>,
    // children: Vec<ASTChild>,
}

pub fn parse(stream: TokenStream) -> Result<Option<Root>> {
    if stream.is_empty() {
        return Ok(None);
    }

    let grammar = Grammar::new();
    grammar.parse(stream)
}

struct Grammar {
    LeftAngle: Punct,
    RightAngle: Punct,
    ForwardSlash: Punct,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            LeftAngle: Punct::new('<', Spacing::Alone),
            RightAngle: Punct::new('>', Spacing::Alone),
            ForwardSlash: Punct::new('/', Spacing::Alone),
        }
    }

    pub fn parse(&self, stream: TokenStream) -> Result<Option<Root>> {
        if stream.is_empty() {
            return Ok(None);
        }

        let mut input = TokenIterator::new(stream);
        self.parse_root(&mut input).and_then(|ast| Ok(Some(ast)))
    }

    fn parse_root(&self, input: &mut TokenIterator) -> Result<Root> {
        if input.is_next_punct(&self.LeftAngle) {
            self.parse_node(input).and_then(|node| Ok(Root::Node(node)))
        } else {
            Err(ASTError::UnexpectedInput)
        }
    }

    fn parse_node(&self, input: &mut TokenIterator) -> Result<Node> {
        input.chomp_punct(&self.LeftAngle)?;
        let tag = input.peek().unwrap().to_string();
        input.chomp()?;

        let mut is_self_closing = false;
        if input.is_next_punct(&self.ForwardSlash) {
            input.chomp_punct(&self.ForwardSlash)?;
            input.chomp_punct(&self.RightAngle)?;

            is_self_closing = true;
        }

        Ok(Node {
            tag,
            is_self_closing,
        })
    }
}

#[cfg(test)]
mod parse {
    use super::*;
    use ::quote::quote;

    #[test]
    fn it_should_return_node_for_self_closing_tag() -> Result<()> {
        let code = quote! {
          <div/>
        };

        let nodes = parse(code.into())?.unwrap();
        assert_eq!(
            nodes,
            Root::Node(Node {
                tag: "div".to_string(),
                is_self_closing: true,
            })
        );

        Ok(())
    }
}
