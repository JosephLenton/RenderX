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

mod micro_vec;
use self::micro_vec::MicroVec;

const EXCLAMATION_MARK: char = '!';
const HYPHEN: char = '-';
const LEFT_ANGLE: char = '<';
const RIGHT_ANGLE: char = '>';
const FORWARD_SLASH: char = '/';
const EQUALS: char = '=';

static COMMENT_CLOSING_LOOKAHEAD: &'static [char] = &[HYPHEN, HYPHEN, RIGHT_ANGLE];
static TAG_OPENING_LOOKAHEAD: &'static [char] = &[LEFT_ANGLE];
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
    let mut nodes = MicroVec::new();

    while !input.is_empty() {
        let node = parse_node(input)?;
        nodes.push(node);
    }

    match nodes {
        MicroVec::None => Err(Error::EmptyMacroStreamGiven),
        MicroVec::Item(node) => Ok(node),
        MicroVec::Vec(children) => Ok(Node::Fragment { children }),
    }
}

fn parse_node(input: &mut TokenIterator) -> Result<Node> {
    if input.is_next_punct(LEFT_ANGLE) {
        if input.is_lookahead_punct(EXCLAMATION_MARK, 1) {
            if input.is_lookahead_punct(HYPHEN, 2) {
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
    input.chomp_puncts(&[LEFT_ANGLE, EXCLAMATION_MARK, HYPHEN, HYPHEN])?;

    let children = parse_comment_children(input)?;

    input.chomp_puncts(&[HYPHEN, HYPHEN, RIGHT_ANGLE])?;

    Ok(Node::Comment { children })
}

fn parse_comment_children(input: &mut TokenIterator) -> Result<Option<Vec<Node>>> {
    let mut maybe_children = None;

    loop {
        if input.is_empty() {
            return Err(Error::MoreTokensExpected);
        }

        if input.is_lookahead_puncts(&COMMENT_CLOSING_LOOKAHEAD) {
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
    input.chomp_puncts(&[LEFT_ANGLE, EXCLAMATION_MARK])?;
    let name = input.chomp_ident()?;
    let attributes = parse_attributes(input)?;
    input.chomp_punct(RIGHT_ANGLE)?;

    Ok(Node::Doctype { name, attributes })
}

fn parse_node_tag(input: &mut TokenIterator) -> Result<Node> {
    input.chomp_punct(LEFT_ANGLE)?;

    // parses </>
    if input.is_lookahead_puncts(&[FORWARD_SLASH, RIGHT_ANGLE]) {
        input.chomp_puncts(&[FORWARD_SLASH, RIGHT_ANGLE])?;
        return Ok(Node::Empty);
    }

    // parses <>(... contents)</>
    if input.is_next_punct(RIGHT_ANGLE) {
        input.chomp_punct(RIGHT_ANGLE)?;
        let maybe_children = parse_children(input)?;
        input.chomp_puncts(&[LEFT_ANGLE, FORWARD_SLASH, RIGHT_ANGLE])?;

        return match maybe_children {
            Some(children) => Ok(Node::Fragment { children }),
            None => Ok(Node::Empty),
        };
    }

    // Real tags from here on. i.e. <div></div> and <hr />
    let opening_tag_name = input.chomp_ident()?;

    let attributes = parse_attributes(input)?;

    if input.is_next_punct(FORWARD_SLASH) {
        input.chomp_puncts(&[FORWARD_SLASH, RIGHT_ANGLE])?;

        return Ok(Node::SelfClosing {
            name: opening_tag_name,
            attributes,
        });
    }

    input.chomp_punct(RIGHT_ANGLE)?;

    let children = parse_children(input)?;

    // Closing Tag.
    input.chomp_puncts(&[LEFT_ANGLE, FORWARD_SLASH])?;
    let closing_tag_name = input.chomp_ident()?;
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
    Ok(Node::Text(parse_text(input, &TAG_OPENING_LOOKAHEAD)?))
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
    let maybe_key = parse_attribute_key(input)?;
    if maybe_key.is_none() {
        return Ok(None);
    }

    let key = maybe_key.unwrap();
    if input.is_next_punct(EQUALS) {
        input.chomp_punct(EQUALS)?;
        let value = Some(parse_attribute_value(input)?);
        return Ok(Some(Attribute { key, value }));
    }

    Ok(Some(Attribute { key, value: None }))
}

fn parse_attribute_key(input: &mut TokenIterator) -> Result<Option<String>> {
    let mut maybe_key: Option<String> = None;

    if input.is_next_literal() {
        return Ok(Some(input.chomp_literal()?));
    }

    loop {
        while input.is_next_punct(HYPHEN) {
            match maybe_key.as_mut() {
                Some(key) => write!(key, "{}", input.chomp()?)?,
                None => maybe_key = Some(input.chomp()?.to_string()),
            }
        }

        if input.is_next_ident() {
            match maybe_key.as_mut() {
                Some(key) => write!(key, "{}", input.chomp()?)?,
                None => maybe_key = Some(input.chomp()?.to_string()),
            }
        }

        // Check if there is a `-name` ahead of us.
        // If there is we will capture that too.
        let mut i = 0;
        while input.is_lookahead_punct(HYPHEN, i) {
            i += 1;
        }

        if i > 0 && input.is_lookahead_ident(i) {
            continue;
        }

        break;
    }

    Ok(maybe_key)
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

        if input.is_lookahead_puncts(&TAG_CLOSING_LOOKAHEAD) {
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
        && !input.is_lookahead_puncts(stopping_lookaheads)
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
    mod doctype {
        use super::*;

        #[test]
        fn it_should_render_doctype_html() -> Result<()> {
            let code = quote! {
                <!doctype html>
            };

            let expected = Node::Doctype {
                name: "doctype".to_string(),
                attributes: Some(vec![Attribute {
                    key: "html".to_string(),
                    value: None,
                }]),
            };

            assert_eq_nodes(code, expected)
        }

        #[test]
        fn it_should_preserve_capitalisation() -> Result<()> {
            let code = quote! {
                <!DoCtYpE html>
            };

            let expected = Node::Doctype {
                name: "DoCtYpE".to_string(),
                attributes: Some(vec![Attribute {
                    key: "html".to_string(),
                    value: None,
                }]),
            };

            assert_eq_nodes(code, expected)
        }
    }

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
                    {"this is some code"}
                    "this is another string"
                -->
            };

            let expected = Node::Comment {
                children: Some(vec![
                    Node::Text("this is a <div> </ hr> comment".to_string()),
                    Node::Code(quote! {"this is some code"}),
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

    #[cfg(test)]
    mod fragments {
        use super::*;
        #[test]
        fn it_should_capture_self_closing_blank_nodes() -> Result<()> {
            let code = quote! {
                </>
            };

            let expected = Node::Empty;

            assert_eq_nodes(code, expected)
        }

        #[test]
        fn it_should_capture_blank_nodes() -> Result<()> {
            let code = quote! {
                <></>
            };

            let expected = Node::Empty;

            assert_eq_nodes(code, expected)
        }

        #[test]
        pub fn it_should_render_the_contents_of_fragments() -> Result<()> {
            let code = quote! {
              <>
                <h1>This is a heading</h1>
                This is some text
                <hr />
              </>
            };

            let expected = Node::Fragment {
                children: vec![
                    Node::Open {
                        name: "h1".to_string(),
                        attributes: None,
                        children: Some(vec![Node::Text("This is a heading".to_string())]),
                    },
                    Node::Text("This is some text".to_string()),
                    Node::SelfClosing {
                        name: "hr".to_string(),
                        attributes: None,
                    },
                ],
            };

            assert_eq_nodes(code, expected)
        }
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

        let error = parse(code.into()).err().unwrap();
        assert_eq!(error, Error::MismatchedTagName);
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
                value: Some(AttributeValue::Code(quote! {
                    base_class.child("el")
                })),
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
            children: Some(vec![Node::Code(quote! {
                if foo {
                    &"blah"
                } else {
                    &"foobar"
                }
            })]),
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

    #[cfg(test)]
    mod root_fragments {
        use super::*;
        use ::quote::quote;

        #[test]
        fn it_should_return_fragment_if_content_after_html_in_root() -> Result<()> {
            let code = quote! {
                <h1>
                    "Upgrade today!"
                </h1>
                blah blah
            };

            let expected = Node::Fragment {
                children: vec![
                    Node::Open {
                        name: "h1".to_string(),
                        attributes: None,
                        children: Some(vec![Node::Text("Upgrade today!".to_string())]),
                    },
                    Node::Text("blah blah".to_string()),
                ],
            };

            assert_eq_nodes(code, expected)
        }

        #[test]
        fn it_should_return_fragment_if_html_after_content_in_root() -> Result<()> {
            let code = quote! {
                blah blah
                <h1>
                    "Upgrade today!"
                </h1>
            };

            let expected = Node::Fragment {
                children: vec![
                    Node::Text("blah blah".to_string()),
                    Node::Open {
                        name: "h1".to_string(),
                        attributes: None,
                        children: Some(vec![Node::Text("Upgrade today!".to_string())]),
                    },
                ],
            };

            assert_eq_nodes(code, expected)
        }
    }

    fn assert_eq_nodes(tokens: TokenStream, expected_nodes: Node) -> Result<()> {
        let nodes = parse(tokens.into())?;
        assert_eq!(nodes, expected_nodes);

        Ok(())
    }
}
