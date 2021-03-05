use ::classnames::classname;
use ::core::render::render;
use ::renderx::rsx;

#[cfg(test)]
mod doctype {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_doctype_html() {
        let code = rsx! {
          <!doctype html>
        };

        let html = render(code);
        assert_eq!("<!doctype html>", html);
    }

    #[test]
    fn it_should_preserve_doctype_capitalisation() {
        let code = rsx! {
          <!DoCtYpE html>
        };

        let html = render(code);
        assert_eq!("<!DoCtYpE html>", html);
    }
}

#[cfg(test)]
mod comments {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_empty_comments() {
        let code = rsx! {
          <!-- -->
        };

        let html = render(code);
        assert_eq!("<!-- -->", html);
    }

    #[test]
    fn it_should_render_comments_containing_strings() {
        let code = rsx! {
          <!-- "this is a comment" -->
        };

        let html = render(code);
        assert_eq!("<!-- this is a comment -->", html);
    }
}

#[cfg(test)]
mod fragments {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_empty_nodes_as_an_empty_string() {
        let code = rsx! {
          <></>
        };

        let html = render(code);
        assert_eq!("", html);
    }

    #[test]
    fn it_should_render_empty_self_closing_nodes_as_an_empty_string() {
        let code = rsx! {
          </>
        };

        let html = render(code);
        assert_eq!("", html);
    }

    #[test]
    fn it_should_render_the_contents_of_fragments() {
        let code = rsx! {
          <>
            <h1>This is a heading</h1>
            This is some text
            <hr />
          </>
        };

        let html = render(code);
        assert_eq!("<h1>This is a heading</h1>This is some text<hr/>", html);
    }
}

#[cfg(test)]
mod nodes {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_self_closing_nodes_to_a_string() {
        let code = rsx! {
          <hr/>
        };

        let html = render(code);
        assert_eq!("<hr/>", html);
    }

    #[test]
    fn it_should_render_simple_nodes_to_a_string() {
        let code = rsx! {
          <div></div>
        };

        let html = render(code);
        assert_eq!("<div></div>", html);
    }

    #[test]
    fn it_should_render_simple_nodes_with_unquoted_strings() {
        let code = rsx! {
          <h1>
            hello world!
          </h1>
        };

        let html = render(code);
        assert_eq!("<h1>hello world!</h1>", html);
    }
}

#[cfg(test)]
mod attributes {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_support_key_value_attributes_on_nodes() {
        let code = rsx! {
          <input type="text" />
        };

        let html = render(code);
        assert_eq!("<input type=\"text\"/>", html);
    }

    #[test]
    fn it_should_render_attribute_if_option_some() {
        let input_type = Some("text");
        let code = rsx! {
          <input type={input_type} />
        };

        let html = render(code);
        assert_eq!("<input type=\"text\"/>", html);
    }

    #[test]
    fn it_should_not_render_attribute_if_option_none() {
        let input_type : Option<&'static str> = None;
        let code = rsx! {
          <input type={input_type} />
        };

        let html = render(code);
        assert_eq!("<input/>", html);
    }

    #[test]
    fn it_should_support_key_value_as_number() {
        let code = rsx! {
          <input type="text" min={0} />
        };

        let html = render(code);
        assert_eq!("<input type=\"text\" min=\"0\"/>", html);
    }

    #[test]
    fn it_should_render_attribute_if_bool_true() {
        let code = rsx! {
          <button disabled={true}>Click me</button>
        };

        let html = render(code);
        assert_eq!("<button disabled>Click me</button>", html);
    }

    #[test]
    fn it_should_not_render_attribute_if_bool_false() {
        let code = rsx! {
          <button disabled={false}>Click me</button>
        };

        let html = render(code);
        assert_eq!("<button>Click me</button>", html);
    }

    #[test]
    fn it_should_support_hyphens_in_attribute_names() {
        let code = rsx! {
          <button data-name="MrButton">Click me</button>
        };

        let html = render(code);
        assert_eq!("<button data-name=\"MrButton\">Click me</button>", html);
    }

    #[test]
    fn it_should_support_hyphens_before_attribute_keys() {
        let code = rsx! {
          <button --data-name="MrButton">Click me</button>
        };

        let html = render(code);
        assert_eq!("<button --data-name=\"MrButton\">Click me</button>", html);
    }

    #[test]
    fn it_should_support_multiple_single_attribute_keys_in_a_row() {
        let code = rsx! {
          <button disabled inert>Click me</button>
        };

        let html = render(code);
        assert_eq!("<button disabled inert>Click me</button>", html);
    }

    #[test]
    fn it_should_support_multiple_attribute_keys_with_hyphens_in_a_row() {
        let code = rsx! {
          <button data-js-track data-disabled data-name="MrButton">Click me</button>
        };

        let html = render(code);
        assert_eq!("<button data-js-track data-disabled data-name=\"MrButton\">Click me</button>", html);
    }
}

#[cfg(test)]
mod code {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_the_result_of_an_expression_returning_an_str() {
        let code = rsx! {
          <h1>{"Hello"}</h1>
        };

        let html = render(code);
        assert_eq!("<h1>Hello</h1>", html);
    }

    #[test]
    fn it_should_render_with_str_variables() {
        let text = " yay!";
        let code = rsx! {
          <h1>
            {"Hello world. "}
            This is working,
            {text}
          </h1>
        };

        let html = render(code);
        assert_eq!("<h1>Hello world. This is working, yay!</h1>", html);
    }

    #[test]
    fn it_should_render_with_a_child_of_rsx() {
        let code = rsx! {
          <div>
            pre
            {rsx! {
              <h1>I am a heading</h1>
            }}
            post
          </div>
        };

        let html = render(code);
        assert_eq!("<div>pre<h1>I am a heading</h1>post</div>", html);
    }
}
