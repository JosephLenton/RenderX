use ::proc_macro2::Delimiter;
use ::proc_macro2::Group;
use ::proc_macro2::Ident;
use ::proc_macro2::Literal;
use ::proc_macro2::Punct;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;

pub(crate) fn token_stream_eq(a_stream: &TokenStream, b_stream: &TokenStream) -> bool {
    match (a_stream.is_empty(), b_stream.is_empty()) {
        (true, true) => return true,
        (true, false) => return false,
        (false, true) => return false,
        _ => { /* do nothing */ }
    }

    let mut a_iter = a_stream.clone().into_iter();
    let mut b_iter = b_stream.clone().into_iter();

    let mut a_item = a_iter.next();
    let mut b_item = b_iter.next();

    while a_item.is_some() && b_item.is_some() {
        if !token_tree_eq(a_item.unwrap(), b_item.unwrap()) {
            return false;
        }

        a_item = a_iter.next();
        b_item = b_iter.next();
    }

    a_item.is_none() && b_item.is_none()
}

pub(crate) fn token_tree_eq(a_token_tree: TokenTree, b_token_tree: TokenTree) -> bool {
    match (a_token_tree, b_token_tree) {
        (TokenTree::Punct(a), TokenTree::Punct(b)) => punct_eq(a, b),
        (TokenTree::Ident(a), TokenTree::Ident(b)) => punct_ident(a, b),
        (TokenTree::Group(a), TokenTree::Group(b)) => punct_group(a, b),
        (TokenTree::Literal(a), TokenTree::Literal(b)) => punct_literal(a, b),
        _ => false,
    }
}

pub(crate) fn punct_eq(a: Punct, b: Punct) -> bool {
    a.as_char() == b.as_char() && a.spacing() == b.spacing()
}

pub(crate) fn punct_ident(a: Ident, b: Ident) -> bool {
    a == b
}

pub(crate) fn punct_group(a: Group, b: Group) -> bool {
    a.delimiter() == b.delimiter() && token_stream_eq(&a.stream(), &b.stream())
}

pub(crate) fn punct_literal(a: Literal, b: Literal) -> bool {
    format!("{}", a) == format!("{}", b)
}
