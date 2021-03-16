use ::classnames::classname;
use ::pretty_assertions::assert_eq;
use ::renderx::component;
use ::renderx::dom::Node;
use ::renderx::render::render;
use ::renderx::rsx;
use ::renderx::*;

#[test]
fn it_should_render_self_closing_components_with_no_props() -> Result<(), std::fmt::Error> {
    #[component]
    fn HorizontalRule() -> Node {
        rsx! {
            <hr class="horizontal-rule"/>
        }
    }

    let html = render(rsx! {
        <HorizontalRule />
    })?;

    assert_eq!(html, "<hr class=\"horizontal-rule\"/>");

    Ok(())
}

#[test]
fn it_should_render_self_closing_components_with_props() -> Result<(), std::fmt::Error> {
    struct HorizontalRuleProps {
        class: &'static str,
    }

    #[component]
    fn HorizontalRule(props: HorizontalRuleProps) -> Node {
        rsx! {
            <hr class={props.class} />
        }
    }

    let html = render(rsx! {
        <HorizontalRule class="my-horizontal-rule" />
    })?;

    assert_eq!(html, "<hr class=\"my-horizontal-rule\"/>");

    Ok(())
}

#[test]
fn it_should_support_default_props_and_not_render_when_missing() -> Result<(), std::fmt::Error> {
    #[derive(Default)]
    struct ButtonProps {
        text: Option<&'static str>,
        disabled: bool,
    }

    #[component]
    fn Button(props: ButtonProps) -> Node {
        rsx! {
            <button disabled={props.disabled}>{props.text}</button>
        }
    }

    let html = render(rsx! {
        <Button />
    })?;

    assert_eq!(html, "<button></button>");

    Ok(())
}

#[test]
fn it_should_support_default_props_and_render_when_present() -> Result<(), std::fmt::Error> {
    #[derive(Default)]
    struct ButtonProps {
        text: Option<&'static str>,
        disabled: bool,
    }

    #[component]
    fn Button(props: ButtonProps) -> Node {
        rsx! {
            <button disabled={props.disabled}>{props.text}</button>
        }
    }

    let html = render(rsx! {
        <Button disabled text="Click me" />
    })?;

    assert_eq!(html, "<button disabled>Click me</button>");

    Ok(())
}
