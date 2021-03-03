use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

use crate::ast::Attribute;
use crate::ast::AttributeValue;
use crate::ast::Node;
use crate::error::Result;

pub fn build(ast: Node) -> TokenStream {
    visit_node(ast)
}

fn visit_node(node: Node) -> TokenStream {
    match node {
        Node::Empty => {
            quote! {
              ::renderx::dom::Node::Empty
            }
        }
        Node::Doctype { name, attributes } => {
            let attribute_tokens = visit_optional_attributes(attributes);

            quote! {
                ::renderx::dom::Node::Doctype {
                  name: #name,
                  attributes: #attribute_tokens,
                }
            }
        }
        Node::Fragment { children } => {
            let children_tokens = visit_children(children);

            quote! {
                ::renderx::dom::Node::Fragment {
                    children: #children_tokens
                }
            }
        }
        Node::Comment { children } => {
            let children_tokens = visit_optional_children(children);

            quote! {
                ::renderx::dom::Node::Comment {
                    children: #children_tokens
                }
            }
        }
        Node::SelfClosing { name, attributes } => {
            let attribute_tokens = visit_optional_attributes(attributes);

            quote! {
                ::renderx::dom::Node::new_self_closing(#name, #attribute_tokens)
            }
        }
        Node::Open {
            name,
            attributes,
            children,
        } => {
            let attribute_tokens = visit_optional_attributes(attributes);
            let children_tokens = visit_optional_children(children);

            quote! {
                ::renderx::dom::Node::new_open(#name, #attribute_tokens, #children_tokens)
            }
        }
        Node::Text(text) => {
            quote! {
                ::renderx::dom::Node::Text {
                    contents: #text,
                }
            }
        }
        Node::Code(code) => {
            unimplemented!();
            quote! {
                #code
            }
        }
    }
}

fn visit_optional_attributes(maybe_attributes: Option<Vec<Attribute>>) -> TokenStream {
    match maybe_attributes {
        None => quote! { None },
        Some(attributes) => {
            let tokens = visit_attributes(attributes);

            quote! {
                Some(#tokens)
            }
        }
    }
}

fn visit_attributes(attributes: Vec<Attribute>) -> TokenStream {
    let attribute_tokens: Vec<TokenStream> =
        attributes.into_iter().map(|a| visit_attribute(a)).collect();

    quote! {
        vec![
            #(#attribute_tokens),*
        ]
    }
}

fn visit_attribute(attribute: Attribute) -> TokenStream {
    let key = attribute.key;
    let value = attribute.value;

    match value {
        None => {
            quote! {
                ::renderx::dom::Attribute::new(#key, None)
            }
        }
        Some(AttributeValue::Text(text)) => {
            quote! {
                ::renderx::dom::Attribute::new(#key, Some(#text))
            }
        }
        Some(AttributeValue::Code(code)) => {
            quote! {
                ::renderx::dom::Attribute::new(#key, Some(#code))
            }
        }
    }
}

fn visit_optional_children(maybe_children: Option<Vec<Node>>) -> TokenStream {
    match maybe_children {
        None => quote! { Option::<Vec<::renderx::dom::Node>>::None },
        Some(children) => {
            let tokens = visit_children(children);

            quote! {
                Some(#tokens)
            }
        }
    }
}

fn visit_children(children: Vec<Node>) -> TokenStream {
    let children_tokens: Vec<TokenStream> = children.into_iter().map(|a| visit_node(a)).collect();

    quote! {
        vec![
            #(#children_tokens),*
        ]
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_output_simple_self_closing_nodes() {
        let code = build(Node::SelfClosing {
            name: "hr".to_string(),
            attributes: None,
        });

        let expected = quote! {
          ::renderx::dom::Node::new_self_closing("hr", None)
        };

        assert_eq!(expected.to_string(), code.to_string());
    }

    #[test]
    fn it_should_output_simple_open_nodes() {
        let code = build(Node::Open {
            name: "div".to_string(),
            attributes: None,
            children: None,
        });

        let expected = quote! {
          ::renderx::dom::Node::new_open("div", None, None)
        };

        assert_eq!(expected.to_string(), code.to_string());
    }

    #[test]
    fn it_should_output_nodes_with_literals() {
        let code = build(Node::Open {
            name: "h1".to_string(),
            attributes: None,
            children: Some(vec![Node::Text("hello world!".to_string())]),
        });

        let expected = quote! {
          ::renderx::dom::Node::new_open("h1", None, Some(vec![
            ::renderx::dom::Node::Text {
              contents: "hello world!",
            }
          ]))
        };

        assert_eq!(expected.to_string(), code.to_string());
    }
}
