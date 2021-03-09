use ::classnames::classname;
use ::core::render::render;
use ::pretty_assertions::assert_eq;
use ::renderx::rsx;
use ::renderx::component;

#[component]
fn Banner() {

}

#[test]
fn it_should_render_example_front_page() {
    Banner();
}
