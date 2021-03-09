use ::classnames::classname;
use ::core::render::render;
use ::pretty_assertions::assert_eq;
use ::renderx::rsx;

#[test]
fn it_should_render_example_front_page() {
    const COPY_TITE: &'static str = "Example Page";
    const COPYRIGHT: &'static str = "Copyright Big Inc 2021";

    let html = render(rsx! {
          <!doctype html>
          <html lang="en">
            <head>
              <title>Example Page</title>

    <!-- "
  yo yo yo, get in touch if you fancy a job!
  I got lots of roles available.
  Like baking, and stuff.
" -->
            </head>

            <body>
              <header>
                <h1>{COPY_TITE}</h1>
              </header>

              <article>
              </article>

              <footer>
                <p class="p p--small">
                  {COPYRIGHT}
                </p>
              </footer>
            </body>
          </html>
        });

    assert_eq!("<!doctype html><html lang=\"en\"><head><title>Example Page</title><!-- \n  yo yo yo, get in touch if you fancy a job!\n  I got lots of roles available.\n  Like baking, and stuff.\n --></head><body><header><h1>Example Page</h1></header><article></article><footer><p class=\"p p--small\">Copyright Big Inc 2021</p></footer></body></html>", html);
}
