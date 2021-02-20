use crate::ASTError;
use crate::TokenIterator;
use ::proc_macro2::Ident;
use ::proc_macro2::Punct;
use ::proc_macro2::Spacing;
use ::proc_macro2::Span;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::fmt::Write;

pub type Result<N> = ::std::result::Result<N, ASTError>;

#[derive(Clone, PartialEq, Debug)]
pub enum Root {
    Node(Node),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    key: String,
    value: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    tag: String,
    is_self_closing: bool,
    attributes: Option<Vec<Attribute>>,
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
    LeftBrace: Punct,
    RightBrace: Punct,
    LeftAngle: Punct,
    RightAngle: Punct,
    ForwardSlash: Punct,
    Equals: Punct,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            LeftBrace: Punct::new('{', Spacing::Alone),
            RightBrace: Punct::new('}', Spacing::Alone),
            LeftAngle: Punct::new('<', Spacing::Alone),
            RightAngle: Punct::new('>', Spacing::Alone),
            ForwardSlash: Punct::new('/', Spacing::Alone),
            Equals: Punct::new('=', Spacing::Alone),
        }
    }

    pub fn parse(&self, stream: TokenStream) -> Result<Option<Root>> {
        if stream.is_empty() {
            return Ok(None);
        }

        let mut input = TokenIterator::new(stream);
        let root = self.parse_root(&mut input).and_then(|ast| Ok(Some(ast)));

        if !input.is_empty() {
            eprintln!("{:#?}", input);
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

        // Attributes
        let attributes = self.parse_attributes(input)?;

        if input.is_next_punct(&self.ForwardSlash) {
            input.chomp_punct(&self.ForwardSlash)?;
            input.chomp_punct(&self.RightAngle)?;

            return Ok(Node {
                tag: opening_tag,
                is_self_closing: true,
                attributes,
            });
        }

        input.chomp_punct(&self.RightAngle)?;

        // todo Children

        // Closing Tag.
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
            attributes,
        })
    }

    fn parse_attributes(&self, input: &mut TokenIterator) -> Result<Option<Vec<Attribute>>> {
        let mut maybe_attrs = None;

        while let Some(attribute) = self.parse_attribute(input)? {
            match maybe_attrs.as_mut() {
                None => maybe_attrs = Some(vec![attribute]),
                Some(attrs) => attrs.push(attribute),
            }
        }

        Ok(maybe_attrs)
    }

    fn parse_attribute(&self, input: &mut TokenIterator) -> Result<Option<Attribute>> {
        if input.is_next_ident() {
            let key = input.chomp_ident()?;
            let value = if input.is_next_punct(&self.Equals) {
                input.chomp_punct(&self.Equals)?;
                Some(self.parse_attribute_value(input)?)
            } else {
                None
            };

            return Ok(Some(Attribute { key, value }));
        }

        Ok(None)
    }

    fn parse_attribute_value(&self, input: &mut TokenIterator) -> Result<String> {
        if input.is_brace_group() {
            input.chomp_brace_group()
        } else {
            input.chomp_literal()
        }
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
            attributes: None,
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
            attributes: None,
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
            attributes: None,
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
            attributes: None,
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

    #[test]
    fn it_should_parse_lone_attributes() -> Result<()> {
        let code = quote! {
          <button is_disabled></button>
        };

        let expected = Root::Node(Node {
            tag: "button".to_string(),
            is_self_closing: false,
            attributes: Some(vec![Attribute {
                key: "is_disabled".to_string(),
                value: None,
            }]),
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_lone_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
          <button is_disabled />
        };

        let expected = Root::Node(Node {
            tag: "button".to_string(),
            is_self_closing: true,
            attributes: Some(vec![Attribute {
                key: "is_disabled".to_string(),
                value: None,
            }]),
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_attributes() -> Result<()> {
        let code = quote! {
          <button type="input"></button>
        };

        let expected = Root::Node(Node {
            tag: "button".to_string(),
            is_self_closing: false,
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some("input".to_string()),
            }]),
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
            <button type="input" />
        };

        let expected = Root::Node(Node {
            tag: "button".to_string(),
            is_self_closing: true,
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some("input".to_string()),
            }]),
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_code_attributes() -> Result<()> {
        let code = quote! {
            <button type={base_class.child("el")} />
        };

        let expected = Root::Node(Node {
            tag: "button".to_string(),
            is_self_closing: true,
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some("base_class . child (\"el\")".to_string()),
            }]),
        });

        assert_eq_nodes(code, expected)
    }
}
