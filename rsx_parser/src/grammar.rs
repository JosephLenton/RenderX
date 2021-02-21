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
pub enum Node {
    Tag(Tag),
    Code(String),
    Literal(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    key: String,
    value: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tag {
    tag: String,
    is_self_closing: bool,
    attributes: Option<Vec<Attribute>>,
    children: Option<Vec<Node>>,
}

pub fn parse(stream: TokenStream) -> Result<Option<Node>> {
    if stream.is_empty() {
        return Ok(None);
    }

    let grammar = Grammar::new();
    grammar.parse(stream)
}

struct Grammar {
    left_brace: Punct,
    right_brace: Punct,
    left_angle: Punct,
    right_angle: Punct,
    forward_slash: Punct,
    equals: Punct,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            left_brace: Punct::new('{', Spacing::Alone),
            right_brace: Punct::new('}', Spacing::Alone),
            left_angle: Punct::new('<', Spacing::Alone),
            right_angle: Punct::new('>', Spacing::Alone),
            forward_slash: Punct::new('/', Spacing::Alone),
            equals: Punct::new('=', Spacing::Alone),
        }
    }

    pub fn parse(&self, stream: TokenStream) -> Result<Option<Node>> {
        if stream.is_empty() {
            return Ok(None);
        }

        let mut input = TokenIterator::new(stream);
        let node = self.parse_node(&mut input).and_then(|ast| Ok(Some(ast)));

        if !input.is_empty() {
            return Err(ASTError::ExcessNodesFound);
        }

        node
    }

    fn parse_node(&self, input: &mut TokenIterator) -> Result<Node> {
        if input.is_next_punct(&self.left_angle) {
            self.parse_tag(input).and_then(|tag| Ok(Node::Tag(tag)))
        } else {
            Err(ASTError::UnexpectedInput)
        }
    }

    fn parse_tag(&self, input: &mut TokenIterator) -> Result<Tag> {
        input.chomp_punct(&self.left_angle)?;

        let opening_tag = input.chomp_ident_or("")?;

        // Attributes
        let attributes = self.parse_attributes(input)?;

        if input.is_next_punct(&self.forward_slash) {
            input.chomp_punct(&self.forward_slash)?;
            input.chomp_punct(&self.right_angle)?;

            return Ok(Tag {
                tag: opening_tag,
                is_self_closing: true,
                attributes,
                children: None,
            });
        }

        input.chomp_punct(&self.right_angle)?;

        let children = self.parse_children(input)?;

        // Closing Tag.
        input.chomp_punct(&self.left_angle)?;
        input.chomp_punct(&self.forward_slash)?;
        let closing_tag = input.chomp_ident_or("")?;

        input.chomp_punct(&self.right_angle)?;

        if closing_tag != opening_tag {
            return Err(ASTError::MismatchedTagName);
        }

        Ok(Tag {
            tag: opening_tag,
            is_self_closing: false,
            attributes,
            children,
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
            let value = if input.is_next_punct(&self.equals) {
                input.chomp_punct(&self.equals)?;
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

    fn parse_children(&self, input: &mut TokenIterator) -> Result<Option<Vec<Node>>> {
        let mut maybe_children = None;

        loop {
            if input.is_next_punct(&self.left_angle)
                && input.lookahead_punct(&self.forward_slash, 1)
            {
                return Ok(maybe_children);
            }

            let node;
            if input.is_brace_group() {
                node = Node::Code(input.chomp_brace_group()?);
            } else if input.is_next_punct(&self.left_angle) {
                node = Node::Tag(self.parse_tag(input)?);
            } else {
                return Err(ASTError::UnexpectedInput);
            };

            match maybe_children.as_mut() {
                Some(children) => children.push(node),
                None => maybe_children = Some(vec![node]),
            }
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

        let expected = Node::Tag(Tag {
            tag: "".to_string(),
            is_self_closing: true,
            attributes: None,
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_capture_blank_nodes() -> Result<()> {
        let code = quote! {
            <></>
        };

        let expected = Node::Tag(Tag {
            tag: "".to_string(),
            is_self_closing: false,
            attributes: None,
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_node_for_self_closing_tag() -> Result<()> {
        let code = quote! {
          <div/>
        };

        let expected = Node::Tag(Tag {
            tag: "div".to_string(),
            is_self_closing: true,
            attributes: None,
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_node_for_an_empty_tag() -> Result<()> {
        let code = quote! {
          <h1></h1>
        };

        let expected = Node::Tag(Tag {
            tag: "h1".to_string(),
            is_self_closing: false,
            attributes: None,
            children: None,
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

    fn assert_eq_nodes(tokens: TokenStream, expected_nodes: Node) -> Result<()> {
        let nodes = parse(tokens.into())?.unwrap();
        assert_eq!(nodes, expected_nodes,);

        Ok(())
    }

    #[test]
    fn it_should_parse_lone_attributes() -> Result<()> {
        let code = quote! {
          <button is_disabled></button>
        };

        let expected = Node::Tag(Tag {
            tag: "button".to_string(),
            is_self_closing: false,
            attributes: Some(vec![Attribute {
                key: "is_disabled".to_string(),
                value: None,
            }]),
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_lone_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
          <button is_disabled />
        };

        let expected = Node::Tag(Tag {
            tag: "button".to_string(),
            is_self_closing: true,
            attributes: Some(vec![Attribute {
                key: "is_disabled".to_string(),
                value: None,
            }]),
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_attributes() -> Result<()> {
        let code = quote! {
          <button type="input"></button>
        };

        let expected = Node::Tag(Tag {
            tag: "button".to_string(),
            is_self_closing: false,
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some("input".to_string()),
            }]),
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
            <button type="input" />
        };

        let expected = Node::Tag(Tag {
            tag: "button".to_string(),
            is_self_closing: true,
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some("input".to_string()),
            }]),
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_code_attributes() -> Result<()> {
        let code = quote! {
            <button type={base_class.child("el")} />
        };

        let expected = Node::Tag(Tag {
            tag: "button".to_string(),
            is_self_closing: true,
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some("base_class . child (\"el\")".to_string()),
            }]),
            children: None,
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_child_nodes() -> Result<()> {
        let code = quote! {
            <div>
                <h1/>
            </div>
        };

        let expected = Node::Tag(Tag {
            tag: "div".to_string(),
            is_self_closing: false,
            attributes: None,
            children: Some(vec![Node::Tag(Tag {
                tag: "h1".to_string(),
                is_self_closing: true,
                attributes: None,
                children: None,
            })]),
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_multiple_child_nodes() -> Result<()> {
        let code = quote! {
            <div>
                <h1></h1>
                <span>
                    <div></div>
                </span>
                <article />
            </div>
        };

        let expected = Node::Tag(Tag {
            tag: "div".to_string(),
            is_self_closing: false,
            attributes: None,
            children: Some(vec![
                Node::Tag(Tag {
                    tag: "h1".to_string(),
                    is_self_closing: false,
                    attributes: None,
                    children: None,
                }),
                Node::Tag(Tag {
                    tag: "span".to_string(),
                    is_self_closing: false,
                    attributes: None,
                    children: Some(vec![Node::Tag(Tag {
                        tag: "div".to_string(),
                        is_self_closing: false,
                        attributes: None,
                        children: None,
                    })]),
                }),
                Node::Tag(Tag {
                    tag: "article".to_string(),
                    is_self_closing: true,
                    attributes: None,
                    children: None,
                }),
            ]),
        });

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_code_in_a_node() -> Result<()> {
        let code = quote! {
            <div>
                {
                    if foo {
                        &"blah"
                    } else {
                        &"foobar"
                    }
                }
            </div>
        };

        let expected = Node::Tag(Tag {
            tag: "div".to_string(),
            is_self_closing: false,
            attributes: None,
            children: Some(vec![Node::Code(
                "if foo { & \"blah\" } else { & \"foobar\" }".to_string(),
            )]),
        });

        assert_eq_nodes(code, expected)
    }
}
