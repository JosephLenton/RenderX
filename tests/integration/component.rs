use ::classnames::classname;
use ::pretty_assertions::assert_eq;
use ::renderx::component;
use ::renderx::dom::Node;
use ::renderx::render::render;
use ::renderx::rsx;

#[component]
fn HorizontalRule() -> Node {
    rsx! {
        <hr class="horizontal-rule"/>
    }
}

#[test]
fn it_should_render_example_front_page() {
    let html = render(rsx! {
        <h1>"Hello!"</h1>
        <HorizontalRule />
    });

    assert_eq!(html, "<h1>Hello!</h1><hr class=\"horizontal-rule\"/>")
}
