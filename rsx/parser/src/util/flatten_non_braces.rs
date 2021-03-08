use ::proc_macro2::Delimiter;
use ::proc_macro2::Punct;
use ::proc_macro2::Spacing;
use ::proc_macro2::TokenTree;

pub fn flatten_non_braces<I>(stream: I) -> Vec<TokenTree>
where
    I: IntoIterator<Item = TokenTree>,
{
    let mut new_stream = vec![];
    flatten_non_braces_into(&mut new_stream, stream);
    new_stream
}

fn flatten_non_braces_into<I>(new_stream: &mut Vec<TokenTree>, stream: I)
where
    I: IntoIterator<Item = TokenTree>,
{
    for item in stream {
        match item {
            TokenTree::Group(group) => {
                let delimiter = group.delimiter();
                if delimiter != Delimiter::Brace {
                    let (opening_char, closing_char) = delimiter_chars(delimiter);

                    new_stream.push(TokenTree::Punct(Punct::new(opening_char, Spacing::Alone)));
                    flatten_non_braces_into(new_stream, group.stream());
                    new_stream.push(TokenTree::Punct(Punct::new(closing_char, Spacing::Alone)));

                    continue;
                }

                new_stream.push(TokenTree::Group(group));
            }
            _ => new_stream.push(item),
        }
    }
}

fn delimiter_chars(delimiter: Delimiter) -> (char, char) {
    match delimiter {
        Delimiter::Bracket => ('[', ']'),
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::None => ('\0', '\0'),
    }
}

#[cfg(test)]
mod flatten {
    use super::*;
    use ::quote::quote;

    #[test]
    fn it_should_flatten_parenthesis_groups() {
        let tokens = quote! {
            a ( x y z ) c
        };

        let mut input = flatten_non_braces(tokens).into_iter();
        assert!(input.next().unwrap().to_string() == "a");
        assert!(input.next().unwrap().to_string() == "(");
        assert!(input.next().unwrap().to_string() == "x");
        assert!(input.next().unwrap().to_string() == "y");
        assert!(input.next().unwrap().to_string() == "z");
        assert!(input.next().unwrap().to_string() == ")");
        assert!(input.next().unwrap().to_string() == "c");
    }

    #[test]
    fn it_should_flatten_square_bracket_groups() {
        let tokens = quote! {
            a [ x y z ] c
        };

        let mut input = flatten_non_braces(tokens).into_iter();
        assert!(input.next().unwrap().to_string() == "a");
        assert!(input.next().unwrap().to_string() == "[");
        assert!(input.next().unwrap().to_string() == "x");
        assert!(input.next().unwrap().to_string() == "y");
        assert!(input.next().unwrap().to_string() == "z");
        assert!(input.next().unwrap().to_string() == "]");
        assert!(input.next().unwrap().to_string() == "c");
    }

    #[test]
    fn it_should_not_flatten_brace_groups() {
        let tokens = quote! {
            a { x y z } c
        };

        let mut input = flatten_non_braces(tokens).into_iter();
        assert!(input.next().unwrap().to_string() == "a");
        assert!(input.next().unwrap().to_string() == "{ x y z }");
        assert!(input.next().unwrap().to_string() == "c");
    }
}
