use crate::ast::Attribute;
use crate::ast::AttributeValue;
use crate::ast::Node;
use crate::error::Error;
use crate::error::Result;

use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::fmt::Write;

mod token_iterator;
use self::token_iterator::TokenIterator;

const EXCLAMATION_MARK: char = '!';
const HYPHEN: char = '-';
const LEFT_ANGLE: char = '<';
const RIGHT_ANGLE: char = '>';
const FORWARD_SLASH: char = '/';
const EQUALS: char = '=';

static COMMENT_CLOSING_LOOKAHEAD: &'static [char] = &[HYPHEN, HYPHEN, RIGHT_ANGLE];
static TAG_CLOSING_LOOKAHEAD: &'static [char] = &[LEFT_ANGLE, FORWARD_SLASH];

pub fn parse(stream: TokenStream) -> Result<Node> {
    parse_root(stream)
}

fn parse_root(stream: TokenStream) -> Result<Node> {
    if stream.is_empty() {
        return Err(Error::EmptyMacroStreamGiven);
    }

    let mut input = TokenIterator::new(stream);
    let node = parse_root_node(&mut input)?;

    if !input.is_empty() {
        return Err(Error::ExcessNodesFound);
    }

    Ok(node)
}

fn parse_root_node(input: &mut TokenIterator) -> Result<Node> {
    parse_node(input)
}

fn parse_node(input: &mut TokenIterator) -> Result<Node> {
    if input.is_next_punct(LEFT_ANGLE) {
        if input.lookahead_punct(EXCLAMATION_MARK, 1) {
            if input.lookahead_punct(HYPHEN, 2) {
                parse_node_comment(input)
            } else {
                parse_node_doctype(input)
            }
        } else {
            parse_node_tag(input)
        }
    } else if input.is_brace_group() {
        Ok(Node::Code(input.chomp_brace_group()?))
    } else {
        parse_node_text(input)
    }
}

fn parse_node_comment(input: &mut TokenIterator) -> Result<Node> {
    input.chomp_punct(LEFT_ANGLE)?;
    input.chomp_punct(EXCLAMATION_MARK)?;
    input.chomp_punct(HYPHEN)?;
    input.chomp_punct(HYPHEN)?;

    let children = parse_comment_children(input)?;

    input.chomp_punct(HYPHEN)?;
    input.chomp_punct(HYPHEN)?;
    input.chomp_punct(RIGHT_ANGLE)?;

    Ok(Node::Comment { children })
}

fn parse_comment_children(input: &mut TokenIterator) -> Result<Option<Vec<Node>>> {
    let mut maybe_children = None;

    loop {
        if input.is_empty() {
            return Err(Error::MoreTokensExpected);
        }

        if input.lookahead_puncts(&COMMENT_CLOSING_LOOKAHEAD) {
            return Ok(maybe_children);
        }

        let child = if input.is_brace_group() {
            Node::Code(input.chomp_brace_group()?)
        } else {
            Node::Text(parse_text(input, &COMMENT_CLOSING_LOOKAHEAD)?)
        };

        match maybe_children.as_mut() {
            Some(children) => children.push(child),
            None => maybe_children = Some(vec![child]),
        }
    }
}

fn parse_node_doctype(input: &mut TokenIterator) -> Result<Node> {
    Ok(Node::Doctype {
        name: "Doctype".to_string(),
        attributes: None,
    })
}

fn parse_node_tag(input: &mut TokenIterator) -> Result<Node> {
    input.chomp_punct(LEFT_ANGLE)?;

    let opening_tag_name = input.chomp_ident_or("")?;

    // Attributes
    let attributes = parse_attributes(input)?;

    if input.is_next_punct(FORWARD_SLASH) {
        input.chomp_punct(FORWARD_SLASH)?;
        input.chomp_punct(RIGHT_ANGLE)?;

        return Ok(Node::SelfClosing {
            name: opening_tag_name,
            attributes,
        });
    }

    input.chomp_punct(RIGHT_ANGLE)?;

    let children = parse_children(input)?;

    // Closing Tag.
    input.chomp_punct(LEFT_ANGLE)?;
    input.chomp_punct(FORWARD_SLASH)?;
    let closing_tag_name = input.chomp_ident_or("")?;

    input.chomp_punct(RIGHT_ANGLE)?;

    if closing_tag_name != opening_tag_name {
        return Err(Error::MismatchedTagName);
    }

    Ok(Node::Open {
        name: opening_tag_name,
        attributes,
        children,
    })
}

fn parse_node_text(input: &mut TokenIterator) -> Result<Node> {
    Ok(Node::Text(parse_text(input, &TAG_CLOSING_LOOKAHEAD)?))
}

fn parse_attributes(input: &mut TokenIterator) -> Result<Option<Vec<Attribute>>> {
    let mut maybe_attrs = None;

    while let Some(attribute) = parse_attribute(input)? {
        match maybe_attrs.as_mut() {
            None => maybe_attrs = Some(vec![attribute]),
            Some(attrs) => attrs.push(attribute),
        }
    }

    Ok(maybe_attrs)
}

fn parse_attribute(input: &mut TokenIterator) -> Result<Option<Attribute>> {
    if input.is_next_ident() {
        let key = input.chomp_ident()?;
        let value = if input.is_next_punct(EQUALS) {
            input.chomp_punct(EQUALS)?;
            Some(parse_attribute_value(input)?)
        } else {
            None
        };

        return Ok(Some(Attribute { key, value }));
    }

    Ok(None)
}

fn parse_attribute_value(input: &mut TokenIterator) -> Result<AttributeValue> {
    if input.is_brace_group() {
        Ok(AttributeValue::Code(input.chomp_brace_group()?))
    } else {
        Ok(AttributeValue::Text(input.chomp_literal()?))
    }
}

/// Finds and grabs all child nodes, and then returns them in a Vec.
/// `stopping_lookaheads` is for telling it what puncts to look for,
/// to know to stop parsing. For a HTML tag this is `</`, and for a comment this is `-->`.
fn parse_children(input: &mut TokenIterator) -> Result<Option<Vec<Node>>> {
    let mut maybe_children = None;

    loop {
        if input.is_empty() {
            return Err(Error::MoreTokensExpected);
        }

        if input.lookahead_puncts(&TAG_CLOSING_LOOKAHEAD) {
            return Ok(maybe_children);
        }

        let child = parse_node(input)?;

        match maybe_children.as_mut() {
            Some(children) => children.push(child),
            None => maybe_children = Some(vec![child]),
        }
    }
}

fn parse_text(input: &mut TokenIterator, stopping_lookaheads: &[char]) -> Result<String> {
    let mut text = String::new();
    let mut last_spacing_rules = (false, false);

    while !input.is_brace_group()
        && !input.is_empty()
        && !input.lookahead_puncts(stopping_lookaheads)
    {
        let next = input.chomp()?;

        let next_spacing_rules = spacing_rules(&next);
        match (last_spacing_rules, next_spacing_rules) {
            ((_, true), (true, _)) => {
                write!(text, " ")?;
            }
            _ => {}
        }
        last_spacing_rules = next_spacing_rules;

        match next {
            TokenTree::Ident(ident) => {
                write!(text, "{}", ident)?;
            }
            TokenTree::Punct(punct) => {
                write!(text, "{}", punct)?;
            }
            TokenTree::Literal(literal) => {
                let literal_string = literal.to_string();
                if literal_string.starts_with('"') {
                    let literal_substring = &literal_string.as_str()[1..literal_string.len() - 1];
                    write!(text, "{}", literal_substring)?;
                } else {
                    write!(text, "{}", literal_string)?;
                }
            }
            TokenTree::Group(_) => unreachable!(),
        }
    }

    Ok(text)
}

fn spacing_rules(tree: &TokenTree) -> (bool, bool) {
    match tree {
        TokenTree::Ident(_) => (true, true),
        TokenTree::Literal(_) => (true, true),
        TokenTree::Group(_) => (true, true),
        TokenTree::Punct(punct) => char_spacing_rules(punct.as_char()),
    }
}

fn char_spacing_rules(c: char) -> (bool, bool) {
    match c {
        '.' => (false, true),
        ',' => (false, true),
        ';' => (false, true),
        ':' => (false, true),
        '?' => (false, true),
        '!' => (false, true),
        '%' => (false, true),
        ')' => (false, true),
        ']' => (false, true),
        '>' => (false, true),
        '}' => (false, true),
        '(' => (true, false),
        '[' => (true, false),
        '{' => (true, false),
        '<' => (true, false),
        '-' => (false, false),
        _ => (true, true),
    }
}

#[cfg(test)]
mod parse {
    use super::*;
    use ::pretty_assertions::assert_eq;
    use ::quote::quote;

    #[cfg(test)]
    mod comments {
        use super::*;

        #[test]
        fn it_should_support_empty_comments() -> Result<()> {
            let code = quote! {
                <!-- -->
            };

            let expected = Node::Comment { children: None };

            assert_eq_nodes(code, expected)
        }

        #[test]
        fn it_should_support_simple_comments() -> Result<()> {
            let code = quote! {
                <!-- this is a comment -->
            };

            let expected = Node::Comment {
                children: Some(vec![Node::Text("this is a comment".to_string())]),
            };

            assert_eq_nodes(code, expected)
        }

        #[test]
        fn it_should_parse_tags_like_they_are_text() -> Result<()> {
            let code = quote! {
                <!-- this is a <div> </hr> comment -->
            };

            let expected = Node::Comment {
                children: Some(vec![Node::Text(
                    "this is a <div> </ hr> comment".to_string(),
                )]),
            };

            assert_eq_nodes(code, expected)
        }

        #[test]
        fn it_should_support_code() -> Result<()> {
            let code = quote! {
                <!--
                    this is a <div> </hr> comment
                    {&"this is some code"}
                    "this is another string"
                -->
            };

            let expected = Node::Comment {
                children: Some(vec![
                    Node::Text("this is a <div> </ hr> comment".to_string()),
                    Node::Code("& \"this is some code\"".to_string()),
                    Node::Text("this is another string".to_string()),
                ]),
            };

            assert_eq_nodes(code, expected)
        }
    }

    #[test]
    fn it_should_support_root_literals() -> Result<()> {
        let code = quote! {
          blah blah blah
        };

        let expected = Node::Text("blah blah blah".to_string());

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_capture_self_closing_blank_nodes() -> Result<()> {
        let code = quote! {
          </>
        };

        let expected = Node::SelfClosing {
            name: "".to_string(),
            attributes: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_capture_blank_nodes() -> Result<()> {
        let code = quote! {
            <></>
        };

        let expected = Node::Open {
            name: "".to_string(),
            attributes: None,
            children: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_node_for_self_closing_tag() -> Result<()> {
        let code = quote! {
          <div/>
        };

        let expected = Node::SelfClosing {
            name: "div".to_string(),
            attributes: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_node_for_an_empty_tag() -> Result<()> {
        let code = quote! {
          <h1></h1>
        };

        let expected = Node::Open {
            name: "h1".to_string(),
            attributes: None,
            children: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_return_an_error_on_mismatched_closing_node() {
        let code = quote! {
          <div></p>
        };

        let result = parse(code.into());
        assert_eq!(result, Err(Error::MismatchedTagName),);
    }

    #[test]
    fn it_should_parse_lone_attributes() -> Result<()> {
        let code = quote! {
          <button is_disabled></button>
        };

        let expected = Node::Open {
            name: "button".to_string(),
            attributes: Some(vec![Attribute {
                key: "is_disabled".to_string(),
                value: None,
            }]),
            children: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_lone_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
          <button is_disabled />
        };

        let expected = Node::SelfClosing {
            name: "button".to_string(),
            attributes: Some(vec![Attribute {
                key: "is_disabled".to_string(),
                value: None,
            }]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_attributes() -> Result<()> {
        let code = quote! {
          <button type="input"></button>
        };

        let expected = Node::Open {
            name: "button".to_string(),
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some(AttributeValue::Text("input".to_string())),
            }]),
            children: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
            <button type="input" />
        };

        let expected = Node::SelfClosing {
            name: "button".to_string(),
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some(AttributeValue::Text("input".to_string())),
            }]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_key_value_code_attributes() -> Result<()> {
        let code = quote! {
            <button type={base_class.child("el")} />
        };

        let expected = Node::SelfClosing {
            name: "button".to_string(),
            attributes: Some(vec![Attribute {
                key: "type".to_string(),
                value: Some(AttributeValue::Code(
                    "base_class . child (\"el\")".to_string(),
                )),
            }]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_child_nodes() -> Result<()> {
        let code = quote! {
            <div>
                <h1/>
            </div>
        };

        let expected = Node::Open {
            name: "div".to_string(),
            attributes: None,
            children: Some(vec![Node::SelfClosing {
                name: "h1".to_string(),
                attributes: None,
            }]),
        };

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

        let expected = Node::Open {
            name: "div".to_string(),
            attributes: None,
            children: Some(vec![
                Node::Open {
                    name: "h1".to_string(),
                    attributes: None,
                    children: None,
                },
                Node::Open {
                    name: "span".to_string(),
                    attributes: None,
                    children: Some(vec![Node::Open {
                        name: "div".to_string(),
                        attributes: None,
                        children: None,
                    }]),
                },
                Node::SelfClosing {
                    name: "article".to_string(),
                    attributes: None,
                },
            ]),
        };

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

        let expected = Node::Open {
            name: "div".to_string(),
            attributes: None,
            children: Some(vec![Node::Code(
                "if foo { & \"blah\" } else { & \"foobar\" }".to_string(),
            )]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_text_in_a_node() -> Result<()> {
        let code = quote! {
            <h1>
                Upgrade today!
            </h1>
        };

        let expected = Node::Open {
            name: "h1".to_string(),
            attributes: None,
            children: Some(vec![Node::Text("Upgrade today!".to_string())]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_text_and_bracket_in_a_node() -> Result<()> {
        let code = quote! {
            <h1>
                (Upgrade today!)
            </h1>
        };

        let expected = Node::Open {
            name: "h1".to_string(),
            attributes: None,
            children: Some(vec![Node::Text("(Upgrade today!)".to_string())]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_text_and_bracket_in_a_node_complex_example() -> Result<()> {
        let code = quote! {
            <h1>
                You should (Upgrade (to something new) today! + = 5 (maybe)) if you want to
            </h1>
        };

        let expected = Node::Open {
            name: "h1".to_string(),
            attributes: None,
            children: Some(vec![Node::Text(
                "You should (Upgrade (to something new) today! + = 5 (maybe)) if you want to"
                    .to_string(),
            )]),
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_parse_text_with_quotes_in_a_node() -> Result<()> {
        let code = quote! {
            <h1>
                "Upgrade today!"
            </h1>
        };

        let expected = Node::Open {
            name: "h1".to_string(),
            attributes: None,
            children: Some(vec![Node::Text("Upgrade today!".to_string())]),
        };

        assert_eq_nodes(code, expected)
    }

    fn assert_eq_nodes(tokens: TokenStream, expected_nodes: Node) -> Result<()> {
        let nodes = parse(tokens.into())?;
        assert_eq!(nodes, expected_nodes,);

        Ok(())
    }

    #[cfg(test)]
    mod errors {
        use super::*;
        use ::pretty_assertions::assert_eq;
        use ::quote::quote;

        #[test]
        fn it_should_error_if_content_after_html_in_root() {
            let code = quote! {
                <h1>
                    "Upgrade today!"
                </h1>
                blah blah
            };

            let r = parse(code.into());
            assert_eq!(Err(Error::ExcessNodesFound), r);
        }

        #[test]
        fn it_should_error_if_html_after_content_in_root() {
            let code = quote! {
                blah blah
                <h1>
                    "Upgrade today!"
                </h1>
            };

            let r = parse(code.into());
            assert_eq!(Err(Error::ExcessNodesFound), r);
        }
    }
}