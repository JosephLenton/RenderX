use crate::rsx::ast::Attribute;
use crate::rsx::ast::Node;
use crate::rsx::ast::Value;
use crate::rsx::error::Error;
use crate::rsx::error::Result;
use crate::util::flatten_non_braces;
use crate::util::token_stream_eq;
use crate::util::MicroVec;
use crate::util::TokenIterator;

use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::fmt::Write;
use ::std::vec::IntoIter;

const COLON: char = ':';
const EXCLAMATION_MARK: char = '!';
const HYPHEN: char = '-';
const LEFT_ANGLE: char = '<';
const RIGHT_ANGLE: char = '>';
const FORWARD_SLASH: char = '/';
const EQUALS: char = '=';

static COMMENT_CLOSING_LOOKAHEAD: &'static [char] = &[HYPHEN, HYPHEN, RIGHT_ANGLE];
static TAG_OPENING_LOOKAHEAD: &'static [char] = &[LEFT_ANGLE];
static TAG_CLOSING_LOOKAHEAD: &'static [char] = &[LEFT_ANGLE, FORWARD_SLASH];

type TokenIteratorVec = TokenIterator<IntoIter<TokenTree>>;

pub fn parse(stream: TokenStream) -> Result<Node> {
    parse_root(stream)
}

fn parse_root(stream: TokenStream) -> Result<Node> {
    if stream.is_empty() {
        return Err(Error::EmptyMacroStreamGiven);
    }

    let flat_stream = flatten_non_braces(stream);
    let mut input = TokenIterator::new(flat_stream);
    let node = parse_root_node(&mut input)?;

    if !input.is_empty() {
        return Err(Error::ExcessTokensFound);
    }

    Ok(node)
}

fn parse_root_node(input: &mut TokenIteratorVec) -> Result<Node> {
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

fn parse_node(input: &mut TokenIteratorVec) -> Result<Node> {
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

fn parse_node_comment(input: &mut TokenIteratorVec) -> Result<Node> {
    input.chomp_puncts(&[LEFT_ANGLE, EXCLAMATION_MARK, HYPHEN, HYPHEN])?;

    let children = parse_comment_children(input)?;

    input.chomp_puncts(&[HYPHEN, HYPHEN, RIGHT_ANGLE])?;

    Ok(Node::Comment { children })
}

fn parse_comment_children(input: &mut TokenIteratorVec) -> Result<Option<Vec<Node>>> {
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

fn parse_node_doctype(input: &mut TokenIteratorVec) -> Result<Node> {
    input.chomp_puncts(&[LEFT_ANGLE, EXCLAMATION_MARK])?;
    let name = parse_name(input)?;
    let attributes = parse_attributes(input)?;
    input.chomp_punct(RIGHT_ANGLE)?;

    Ok(Node::Doctype { name, attributes })
}

fn parse_node_tag(input: &mut TokenIteratorVec) -> Result<Node> {
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
    let opening_tag_name = parse_name(input)?;
    let attributes = parse_attributes(input)?;
    let is_component = is_component_name(&opening_tag_name);

    if input.is_next_punct(FORWARD_SLASH) {
        input.chomp_puncts(&[FORWARD_SLASH, RIGHT_ANGLE])?;

        if is_component {
            if let Value::Text(opening_tag_name_string) = opening_tag_name {
                return Ok(Node::SelfClosingComponent {
                    name: opening_tag_name_string,
                    attributes,
                });
            } else {
                unreachable!("Component name was not parsed as `Value::Text` (this is a bug)");
            }
        } else {
            return Ok(Node::SelfClosing {
                name: opening_tag_name,
                attributes,
            });
        }
    }

    input.chomp_punct(RIGHT_ANGLE)?;

    let children = parse_children(input)?;

    // Closing Tag.
    input.chomp_puncts(&[LEFT_ANGLE, FORWARD_SLASH])?;
    let closing_tag_name = parse_name(input)?;
    input.chomp_punct(RIGHT_ANGLE)?;

    match (&opening_tag_name, &closing_tag_name) {
        (Value::Text(left_text), Value::Text(right_text)) => {
            if left_text != right_text {
                return Err(Error::MismatchedClosingTagName);
            }
        }
        (Value::Code(left_code), Value::Code(right_code)) => {
            if !right_code.is_empty() {
                if !token_stream_eq(&left_code, &right_code) {
                    return Err(Error::MismatchedClosingTagCode);
                }
            }
        }
        _ => {
            return Err(Error::MismatchedClosingTagName);
        }
    }

    if is_component {
        if let Value::Text(opening_tag_name_string) = opening_tag_name {
            Ok(Node::OpenComponent {
                name: opening_tag_name_string,
                attributes,
                children,
            })
        } else {
            unreachable!("Component name was not parsed as `Value::Text` (this is a bug)");
        }
    } else {
        Ok(Node::Open {
            name: opening_tag_name,
            attributes,
            children,
        })
    }
}

fn parse_node_text(input: &mut TokenIteratorVec) -> Result<Node> {
    Ok(Node::Text(parse_text(input, &TAG_OPENING_LOOKAHEAD)?))
}

fn parse_attributes(input: &mut TokenIteratorVec) -> Result<Option<Vec<Attribute>>> {
    let mut maybe_attrs = None;

    while let Some(attribute) = parse_attribute(input)? {
        match maybe_attrs.as_mut() {
            None => maybe_attrs = Some(vec![attribute]),
            Some(attrs) => attrs.push(attribute),
        }
    }

    Ok(maybe_attrs)
}

fn parse_attribute(input: &mut TokenIteratorVec) -> Result<Option<Attribute>> {
    let maybe_key = parse_maybe_name(input)?;
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

fn parse_attribute_value(input: &mut TokenIteratorVec) -> Result<Value> {
    if input.is_brace_group() {
        Ok(Value::Code(input.chomp_brace_group()?))
    } else {
        Ok(Value::Text(input.chomp_literal()?))
    }
}

fn parse_name(input: &mut TokenIteratorVec) -> Result<Value> {
    match parse_maybe_name(input)? {
        Some(name) => Ok(name),
        None => Err(Error::ExpectedName),
    }
}

fn parse_maybe_name(input: &mut TokenIteratorVec) -> Result<Option<Value>> {
    let mut maybe_key: Option<String> = None;

    if input.is_next_literal() {
        return Ok(Some(Value::Text(input.chomp_literal()?)));
    }

    if input.is_brace_group() {
        return Ok(Some(Value::Code(input.chomp_brace_group()?)));
    }

    loop {
        if input.is_next_ident() {
            let next = input.chomp()?;

            match maybe_key.as_mut() {
                Some(key) => write!(key, "{}", next)?,
                None => maybe_key = Some(next.to_string()),
            }
        }

        // Check if there is `-name` or `:name` ahead of us.
        // If there is we will capture that too.
        let mut i = 0;
        while input.is_lookahead_punct(HYPHEN, i) || input.is_lookahead_punct(COLON, i) {
            i += 1;
        }

        if i > 0 && input.is_lookahead_ident(i) {
            while input.is_next_punct(HYPHEN) || input.is_next_punct(COLON) {
                match maybe_key.as_mut() {
                    Some(key) => write!(key, "{}", input.chomp()?)?,
                    None => maybe_key = Some(input.chomp()?.to_string()),
                }
            }

            continue;
        }

        break;
    }

    Ok(maybe_key.map(|text| Value::Text(text)))
}

/// Finds and grabs all child nodes, and then returns them in a Vec.
/// `stopping_lookaheads` is for telling it what puncts to look for,
/// to know to stop parsing. For a HTML tag this is `</`, and for a comment this is `-->`.
fn parse_children(input: &mut TokenIteratorVec) -> Result<Option<Vec<Node>>> {
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

fn parse_text(input: &mut TokenIteratorVec, stopping_lookaheads: &[char]) -> Result<String> {
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

fn is_component_name(opening_tag_name: &Value) -> bool {
    match opening_tag_name {
        Value::Code(_) => {
            return false;
        }
        Value::Text(name) => {
            let mut chars = name.chars();

            // Starts with an uppercase character.
            match chars.next() {
                None => {
                    return false;
                }
                Some(c) => {
                    if !c.is_ascii_uppercase() {
                        return false;
                    }
                }
            }

            // The rest matches Rust identifier rules.
            for c in chars {
                if !c.is_alphanumeric() && c != '_' {
                    return false;
                }
            }

            true
        }
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
                name: Value::Text("doctype".to_string()),
                attributes: Some(vec![Attribute {
                    key: Value::Text("html".to_string()),
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
                name: Value::Text("DoCtYpE".to_string()),
                attributes: Some(vec![Attribute {
                    key: Value::Text("html".to_string()),
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
                        name: Value::Text("h1".to_string()),
                        attributes: None,
                        children: Some(vec![Node::Text("This is a heading".to_string())]),
                    },
                    Node::Text("This is some text".to_string()),
                    Node::SelfClosing {
                        name: Value::Text("hr".to_string()),
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
            name: Value::Text("div".to_string()),
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
            name: Value::Text("h1".to_string()),
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
        assert_eq!(error, Error::MismatchedClosingTagName);
    }

    #[test]
    fn it_should_parse_lone_attributes() -> Result<()> {
        let code = quote! {
          <button is_disabled></button>
        };

        let expected = Node::Open {
            name: Value::Text("button".to_string()),
            attributes: Some(vec![Attribute {
                key: Value::Text("is_disabled".to_string()),
                value: None,
            }]),
            children: None,
        };

        assert_eq_nodes(code, expected)
    }

    #[test]
    fn it_should_not_support_hyphens_before_attribute_keys() {
        let code = quote! {
          <button --data-name="MrButton">Click me</button>
        };

        let received = parse(code);
        assert_eq!(received.err().unwrap(), Error::UnexpectedToken);
    }

    #[test]
    fn it_should_parse_lone_attributes_on_self_closing_tags() -> Result<()> {
        let code = quote! {
          <button is_disabled />
        };

        let expected = Node::SelfClosing {
            name: Value::Text("button".to_string()),
            attributes: Some(vec![Attribute {
                key: Value::Text("is_disabled".to_string()),
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
            name: Value::Text("button".to_string()),
            attributes: Some(vec![Attribute {
                key: Value::Text("type".to_string()),
                value: Some(Value::Text("input".to_string())),
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
            name: Value::Text("button".to_string()),
            attributes: Some(vec![Attribute {
                key: Value::Text("type".to_string()),
                value: Some(Value::Text("input".to_string())),
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
            name: Value::Text("button".to_string()),
            attributes: Some(vec![Attribute {
                key: Value::Text("type".to_string()),
                value: Some(Value::Code(quote! {
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
            name: Value::Text("div".to_string()),
            attributes: None,
            children: Some(vec![Node::SelfClosing {
                name: Value::Text("h1".to_string()),
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
            name: Value::Text("div".to_string()),
            attributes: None,
            children: Some(vec![
                Node::Open {
                    name: Value::Text("h1".to_string()),
                    attributes: None,
                    children: None,
                },
                Node::Open {
                    name: Value::Text("span".to_string()),
                    attributes: None,
                    children: Some(vec![Node::Open {
                        name: Value::Text("div".to_string()),
                        attributes: None,
                        children: None,
                    }]),
                },
                Node::SelfClosing {
                    name: Value::Text("article".to_string()),
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
            name: Value::Text("div".to_string()),
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
            name: Value::Text("h1".to_string()),
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
            name: Value::Text("h1".to_string()),
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
            name: Value::Text("h1".to_string()),
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
            name: Value::Text("h1".to_string()),
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
                        name: Value::Text("h1".to_string()),
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
                        name: Value::Text("h1".to_string()),
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
