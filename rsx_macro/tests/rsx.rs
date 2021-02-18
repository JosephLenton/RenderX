use ::rsx_macro::rsx;
use ::rsx_core::*;
use ::classnames::classname;
use ::pretty_assertions::{assert_eq};

#[test]
pub fn main() {
  let mut buffer = ServerRender::new();
  component(&mut buffer);

  assert_eq!("<div>hello</div>", buffer.to_string());
}

fn component<R : Render>(
  __rsx_buffer__ : &mut R,
) {
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







  rsx!(
    <div>
      hello
    </div>
  )
}
