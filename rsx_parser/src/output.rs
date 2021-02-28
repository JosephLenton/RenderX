use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

use crate::ast::Attribute;
use crate::ast::AttributeValue;
use crate::ast::Child;
use crate::ast::Node;
use crate::error::Result;

pub fn build(ast: Node) -> TokenStream {
    visit_node(ast)
}

fn visit_node(node: Node) -> TokenStream {
    if node.is_self_closing {
        visit_node_self_closing(node)
    } else {
        visit_node_with_children(node)
    }
}

fn visit_node_self_closing(node: Node) -> TokenStream {
    let name = node.name;
    let attributes = visit_optional_attributes(node.attributes);

    quote! {
      ::rsx_core::dom::Node::new_self_closing(#name, #attributes)
    }
}

fn visit_node_with_children(node: Node) -> TokenStream {
    let name = node.name;
    let attributes = visit_optional_attributes(node.attributes);
    let children = visit_optional_children(node.children);

    quote! {
      ::rsx_core::dom::Node::new(#name, #attributes, #children)
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
    quote! {
      None
    }
}

fn visit_optional_children(maybe_children: Option<Vec<Child>>) -> TokenStream {
    match maybe_children {
        None => quote! { None },
        Some(children) => {
            let tokens = visit_children(children);

            quote! {
              Some(#tokens)
            }
        }
    }
}

fn visit_children(children: Vec<Child>) -> TokenStream {
    let children_tokens: Vec<TokenStream> = children.into_iter().map(|a| visit_child(a)).collect();

    quote! {
      vec![
        #(#children_tokens),*
      ]
    }
}

fn visit_child(child: Child) -> TokenStream {
    match child {
        Child::Text(text) => {
            quote! {
              ::rsx_core::dom::Child::Text(#text)
            }
        }
        Child::Code(code) => {
            quote! {
              #code
            }
        }
        Child::Node(node) => {
            let node_tokens = visit_node(node);
            quote! {
              ::rsx_core::dom::Child::Node(#node_tokens)
            }
        }
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_output_simple_self_closing_nodes() {
        let code = build(Node {
            name: "hr".to_string(),
            is_self_closing: true,
            attributes: None,
            children: None,
        });

        let expected = quote! {
          ::rsx_core::dom::Node::new_self_closing("hr", None)
        };

        assert_eq!(expected.to_string(), code.to_string());
    }

    #[test]
    fn it_should_output_simple_open_nodes() {
        let code = build(Node {
            name: "div".to_string(),
            is_self_closing: false,
            attributes: None,
            children: None,
        });

        let expected = quote! {
          ::rsx_core::dom::Node::new("div", None, None)
        };

        assert_eq!(expected.to_string(), code.to_string());
    }

    #[test]
    fn it_should_output_nodes_with_literals() {
        let code = build(Node {
            name: "h1".to_string(),
            is_self_closing: false,
            attributes: None,
            children: Some(vec![Child::Text("hello world!".to_string())]),
        });

        let expected = quote! {
          ::rsx_core::dom::Node::new("h1", None, Some(vec![
            ::rsx_core::dom::Child::Text("hello world!")
          ]))
        };

        assert_eq!(expected.to_string(), code.to_string());
    }
}
