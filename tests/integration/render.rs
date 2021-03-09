use ::classnames::classname;
use ::core::render::render;
use ::renderx::rsx;

#[cfg(test)]
mod doctype {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_doctype_html() {
        let html = render(rsx! {
          <!doctype html>
        });

        assert_eq!("<!doctype html>", html);
    }

    #[test]
    fn it_should_preserve_doctype_capitalisation() {
        let html = render(rsx! {
          <!DoCtYpE html>
        });

        assert_eq!("<!DoCtYpE html>", html);
    }
}

#[cfg(test)]
mod comments {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_empty_comments() {
        let html = render(rsx! {
          <!-- -->
        });

        assert_eq!("<!-- -->", html);
    }

    #[test]
    fn it_should_render_comments_containing_strings() {
        let html = render(rsx! {
          <!-- "this is a comment" -->
        });

        assert_eq!("<!-- this is a comment -->", html);
    }
}

#[cfg(test)]
mod fragments {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_empty_nodes_as_an_empty_string() {
        let html = render(rsx! {
          <></>
        });

        assert_eq!("", html);
    }

    #[test]
    fn it_should_render_empty_self_closing_nodes_as_an_empty_string() {
        let html = render(rsx! {
          </>
        });

        assert_eq!("", html);
    }

    #[test]
    fn it_should_render_the_contents_of_fragments() {
        let html = render(rsx! {
          <>
            <h1>This is a heading</h1>
            This is some text
            <hr />
          </>
        });

        assert_eq!("<h1>This is a heading</h1>This is some text<hr/>", html);
    }
}

#[cfg(test)]
mod nodes {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_self_closing_nodes_to_a_string() {
        let html = render(rsx! {
          <hr/>
        });

        assert_eq!("<hr/>", html);
    }

    #[test]
    fn it_should_render_simple_nodes_to_a_string() {
        let html = render(rsx! {
          <div></div>
        });

        assert_eq!("<div></div>", html);
    }

    #[test]
    fn it_should_render_simple_nodes_with_unquoted_strings() {
        let html = render(rsx! {
          <h1>
            hello world!
          </h1>
        });

        assert_eq!("<h1>hello world!</h1>", html);
    }

    #[test]
    fn it_should_render_nodes_with_namespaces() {
        let html = render(rsx! {
          <namespace:blah></namespace:blah>
        });

        assert_eq!("<namespace:blah></namespace:blah>", html);
    }

    #[test]
    fn it_should_render_nodes_with_hyphens() {
        let html = render(rsx! {
          <mr-map></mr-map>
        });

        assert_eq!("<mr-map></mr-map>", html);
    }
}

#[cfg(test)]
mod attributes {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_support_key_value_attributes_on_nodes() {
        let html = render(rsx! {
          <input type="text" />
        });

        assert_eq!("<input type=\"text\"/>", html);
    }

    #[test]
    fn it_should_render_attribute_if_option_some() {
        let input_type = Some("text");
        let html = render(rsx! {
          <input type={input_type} />
        });

        assert_eq!("<input type=\"text\"/>", html);
    }

    #[test]
    fn it_should_not_render_attribute_if_option_none() {
        let input_type: Option<&'static str> = None;
        let html = render(rsx! {
          <input type={input_type} />
        });

        assert_eq!("<input/>", html);
    }

    #[test]
    fn it_should_support_key_value_as_number() {
        let html = render(rsx! {
          <input type="text" min={0} />
        });

        assert_eq!("<input type=\"text\" min=\"0\"/>", html);
    }

    #[test]
    fn it_should_render_attribute_if_bool_true() {
        let html = render(rsx! {
          <button disabled={true}>Click me</button>
        });

        assert_eq!("<button disabled>Click me</button>", html);
    }

    #[test]
    fn it_should_not_render_attribute_if_bool_false() {
        let html = render(rsx! {
          <button disabled={false}>Click me</button>
        });

        assert_eq!("<button>Click me</button>", html);
    }

    #[test]
    fn it_should_support_hyphens_in_attribute_names() {
        let html = render(rsx! {
          <button data-name="MrButton">Click me</button>
        });

        assert_eq!("<button data-name=\"MrButton\">Click me</button>", html);
    }

    #[test]
    fn it_should_support_multiple_single_attribute_keys_in_a_row() {
        let html = render(rsx! {
          <button disabled inert>Click me</button>
        });

        assert_eq!("<button disabled inert>Click me</button>", html);
    }

    #[test]
    fn it_should_support_multiple_attribute_keys_with_hyphens_in_a_row() {
        let html = render(rsx! {
          <button data-js-track data-disabled data-name="MrButton">Click me</button>
        });

        assert_eq!(
            "<button data-js-track data-disabled data-name=\"MrButton\">Click me</button>",
            html
        );
    }

    #[test]
    fn it_should_support_text_literals_for_attribute_names() {
        let html = render(rsx! {
          <button "disabled" "data-name"="MrButton" "🌧️"="❤️">Click me</button>
        });

        assert_eq!(
            "<button disabled data-name=\"MrButton\" 🌧️=\"❤️\">Click me</button>",
            html
        );
    }

    #[test]
    fn it_should_render_keys_using_code() {
        let key = "min";
        let html = render(rsx! {
          <input type="text" {key}={0} />
        });

        assert_eq!("<input type=\"text\" min=\"0\"/>", html);
    }

    #[test]
    fn it_should_render_solo_keys_using_code() {
        let attr = "disabled";
        let html = render(rsx! {
          <button {attr}>Click me</button>
        });

        assert_eq!("<button disabled>Click me</button>", html);
    }
}

#[cfg(test)]
mod code {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_render_the_result_of_an_expression_returning_an_str() {
        let html = render(rsx! {
          <h1>{"Hello"}</h1>
        });

        assert_eq!("<h1>Hello</h1>", html);
    }

    #[test]
    fn it_should_render_with_str_variables() {
        let text = " yay!";
        let html = render(rsx! {
          <h1>
            {"Hello world. "}
            This is working,
            {text}
          </h1>
        });

        assert_eq!("<h1>Hello world. This is working, yay!</h1>", html);
    }

    #[test]
    fn it_should_render_with_a_child_of_rsx() {
        let html = render(rsx! {
          <div>
            pre
            {rsx! {
              <h1>I am a heading</h1>
            }}
            post
          </div>
        });

        assert_eq!("<div>pre<h1>I am a heading</h1>post</div>", html);
    }

    #[test]
    fn it_should_render_with_code_providing_component_name() {
        let el = "span";

        let html = render(rsx! {
          <{el}/>
        });

        assert_eq!("<span/>", html);
    }

    #[test]
    fn it_should_render_with_code_name_and_code_closing_tag() {
        let el = "span";

        let html = render(rsx! {
          <{el}></{el}>
        });

        assert_eq!("<span></span>", html);
    }

    #[test]
    fn it_should_render_with_code_name_and_empty_closing_tag() {
        let el = "span";

        let html = render(rsx! {
          <{el}></{}>
        });

        assert_eq!("<span></span>", html);
    }
}
