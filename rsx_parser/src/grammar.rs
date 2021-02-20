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
        let root = self.parse_root(&mut input).and_then(|ast| Ok(Some(ast)));

        if !input.is_empty() {
            return Err(ASTError::ExcessNodesFound);
        }

        root
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

        let opening_tag = input.chomp_ident_or("")?;

        if input.is_next_punct(&self.ForwardSlash) {
            input.chomp_punct(&self.ForwardSlash)?;
            input.chomp_punct(&self.RightAngle)?;

            return Ok(Node {
                tag: opening_tag,
                is_self_closing: true,
            });
        }

        input.chomp_punct(&self.RightAngle)?;

        // todo Children

        input.chomp_punct(&self.LeftAngle)?;
        input.chomp_punct(&self.ForwardSlash)?;
        let closing_tag = input.chomp_ident_or("")?;

        input.chomp_punct(&self.RightAngle)?;

        if closing_tag != opening_tag {
            return Err(ASTError::MismatchedTagName);
        }

        Ok(Node {
            tag: opening_tag,
            is_self_closing: false,
        })
    }
}

#[cfg(test)]
mod parse {
    use super::*;
    use ::quote::quote;

    #[test]
    fn it_should_capture_self_closing_blank_nodes() -> Result<()> {
        let code = quote! {
          </>
        };

        let expected = Root::Node(Node {
            tag: "".to_string(),
            is_self_closing: true,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_capture_blank_nodes() -> Result<()> {
        let code = quote! {
            <></>
        };

        let expected = Root::Node(Node {
            tag: "".to_string(),
            is_self_closing: false,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_node_for_self_closing_tag() -> Result<()> {
        let code = quote! {
          <div/>
        };

        let expected = Root::Node(Node {
            tag: "div".to_string(),
            is_self_closing: true,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_node_for_an_empty_tag() -> Result<()> {
        let code = quote! {
          <h1></h1>
        };

        let expected = Root::Node(Node {
            tag: "h1".to_string(),
            is_self_closing: false,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_an_error_on_mismatched_closing_node() {
        let code = quote! {
          <div></p>
        };

        let result = parse(code.into());
        assert_eq!(result, Err(ASTError::MismatchedTagName),);
    }

    fn assert_eq_nodes(tokens: TokenStream, expected_nodes: Root) -> Result<()> {
        let nodes = parse(tokens.into())?.unwrap();
        assert_eq!(nodes, expected_nodes,);

        Ok(())
    }
}
