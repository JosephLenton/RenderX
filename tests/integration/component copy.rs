use ::classnames::classname;
use ::core::render::render;
use ::pretty_assertions::assert_eq;
use ::renderx::rsx;

#[test]
fn it_should_render_example_front_page() {
    const COPY_TITE: &'static str = "Example Page";
    const COPYRIGHT: &'static str = "Copyright Big Inc 2021";

    let html = render(& rsx! {
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

#[Props]
pub struct OptionProps {
  pub value: &'static str,
}

#[Component]
fn OptionEl(props : OptionProps) -> Node {
  rsx! {
    <option value={props.value}>{props.value}</div>
  }
}

#[Props]
pub struct SelectProps {
  pub options: &[OptionProps]
}

#[Component]
fn Select(props : OptionProps) -> Node {
  rsx! {
    <select>{props.options.map(OptionEl)}</div>
  }
}






// Option 1
#[Props]
pub struct HeadingProps {
  pub text: &'static str,
  pub style: Option<HeadingStyle>,
}

#[Component]
fn Heading(props : HeadingProps) -> Node {
  rsx! {
    <div class="heading">{props.text}</div>
  }
}

#[Component]
fn SalesBanner(store: Store) -> Node {
  rsx! {
    <div class="sales-banner">"Buy today!"</div>
  }
}

fn Page() -> Node {
  rsx! {
    <Heading text="hello"/>
    <SalesBanner />
  }
}

// What you get ...
#[derive(Default)]
pub struct HeadingProps {
  pub text: &'static str,
  pub style: Option<HeadingStyle>,

  pub children: Node,
}

fn Heading<S:Store = ()>(props : HeadingProps, _: S) -> Node {
  rsx! {
    <div class="heading">{props.text}</div>
  }
}

#[Component]
fn SalesBanner(_ : (), store: Store) -> Node {
  rsx! {
    <div class="heading">{props.text}</div>
  }
}

fn Page() -> Node {
  Heading(HeadingProps {
    text: "hello",
    children: Node::StaticText("hello"),
    ..Default::default(),
  })
}





// Option 2
#[Component]
pub struct Heading {
  pub text: &'static str,
  pub style: Option<HeadingStyle>,
}

impl Render for Heading {
  fn render(&self) -> Node {
    rsx! {
      <div class="heading">{self.text}</div>
    }
  }
}

fn Page(store: Store) -> Node {
  rsx! {
    <Heading text="hello"/>
  }
}

// What you get ...
#[derive(Default)]
pub struct Heading<S = ()> {
  pub text: &'static str,
  pub style: Option<HeadingStyle>,

  pub children: Node,
  pub store: S,
}

impl Render for Heading {
  fn render(&self) -> Node {
    rsx! {
      <div class="heading">{self.text}</div>
    }
  }
}

fn Page(store: Store) -> Node {
  Heading {
    text: "hello",
    children: Node::StaticText("hello"),
    store,
    ..Default::default(),
  }.render()
}



