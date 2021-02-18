use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

mod ast;
mod ast_error;
mod grammar;
mod token_iterator;

pub(crate) use self::ast_error::ASTError;
pub(crate) use self::token_iterator::TokenIterator;

pub static BUFFER_NAME: &'static str = "__";

pub fn parse(old_stream: TokenStream) -> TokenStream {
    let stream = TokenStream::from(old_stream);
    let buffer_name = format_ident!("{}", BUFFER_NAME);

    let ast = grammar::parse(stream);

    // let code = quote! {
    //   let r = {
    //     #stream
    //   };

    //   #buffer_name.render(r);
    // };

    // stream
    quote! {}
}

#[derive(Copy, Clone, Debug)]
pub struct Node<'a> {
    tag: &'a str,
    attrs: &'a [Attribute<'a>],
    children: &'a [Child<'a>],
}

#[derive(Copy, Clone, Debug)]
pub enum Child<'a> {
    RawText(&'a str),
    Node(Node<'a>),
}

#[derive(Copy, Clone, Debug)]
pub enum Attribute<'a> {
    Pair(&'a str, &'a str),
    Single(&'a str),
}

#[cfg(test)]
mod parse {
    use super::*;

    #[test]
    fn it_should_return_node_for_self_closing_elements() {
        let code = quote! {
          <div/>
        };

        let nodes = parse(code.into());
        assert_eq!(nodes.to_string(), "blah");
    }
}

// // # Option 2
// buffer.render(
//   Node::new("div", &[], &[
//     &Child::Text("hello"),
//     &Child::Node(Node::new("h1", &[Attr::new(&"class", &"font-h0")], &[
//       &Text("Upgrade Today!"),
//     ]))
//   ])
// );
