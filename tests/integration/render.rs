use ::classnames::classname;
use ::pretty_assertions::assert_eq;
use ::rsx_core::render::render;
use ::rsx_macro::rsx;

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
      <div>
        hello world!
      </div>
    };

    let html = render(comp);
    assert_eq!("<div>hello world!</div>", html);
}

// fn component<R: Render>(__rsx_buffer__: &mut R) {
    // # Example HTML
    // <div>
    //   hello

    //   <h1 class="font-h0">
    //     Upgrade Today!
    //   </h1>
    // </div>

    // // # Option 1
    // buffer.start_open_tag("div");
    // buffer.end_open_tag();
    // buffer.content("hello");

    // buffer.start_open_tag("h1");
    // buffer.attr("class", "font-h0")
    // buffer.end_open_tag();
    // buffer.content("Upgrade Today");
    // buffer.closing_tag("h1");

    // buffer.closing_tag("div");

    // // # Option 2
    // buffer.render(
    //   Node::new("div", &[], &[
    //     &Child::Text("hello"),
    //     &Child::Node(Node::new("h1", &[Attr::new(&"class", &"font-h0")], &[
    //       &Text("Upgrade Today!"),
    //     ]))
    //   ])
    // );

//     rsx!(
//       <div>
//         hello
//       </div>
//     )
// }
