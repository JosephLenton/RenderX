use ::classnames::classname;
use ::core::render::render;
use ::renderx::rsx;

#[cfg(test)]
mod comments {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    pub fn it_should_render_empty_comments() {
        let comp = rsx! {
          <!-- -->
        };

        let html = render(comp);
        assert_eq!("<!-- -->", html);
    }

    #[test]
    pub fn it_should_render_comments_containing_strings() {
        let comp = rsx! {
          <!-- "this is a comment" -->
        };

        let html = render(comp);
        assert_eq!("<!-- this is a comment -->", html);
    }
}

#[cfg(test)]
mod fragments {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    pub fn it_should_render_empty_nodes_as_an_empty_string() {
        let comp = rsx! {
          <></>
        };

        let html = render(comp);
        assert_eq!("", html);
    }

    #[test]
    pub fn it_should_render_empty_self_closing_nodes_as_an_empty_string() {
        let comp = rsx! {
          </>
        };

        let html = render(comp);
        assert_eq!("", html);
    }

    #[test]
    pub fn it_should_render_the_contents_of_fragments() {
        let comp = rsx! {
          <>
            <h1>This is a heading</h1>
            This is some text
            <hr />
          </>
        };

        let html = render(comp);
        assert_eq!("<h1>This is a heading</h1>This is some text<hr/>", html);
    }
}

#[cfg(test)]
mod nodes {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    pub fn it_should_render_self_closing_nodes_to_a_string() {
        let comp = rsx! {
          <hr/>
        };

        let html = render(comp);
        assert_eq!("<hr/>", html);
    }

    #[test]
    pub fn it_should_render_simple_nodes_to_a_string() {
        let comp = rsx! {
          <div></div>
        };

        let html = render(comp);
        assert_eq!("<div></div>", html);
    }

    #[test]
    pub fn it_should_render_simple_nodes_with_unquoted_strings() {
        let comp = rsx! {
          <h1>
            hello world!
          </h1>
        };

        let html = render(comp);
        assert_eq!("<h1>hello world!</h1>", html);
    }
}
