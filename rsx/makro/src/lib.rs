use ::parser::component;
use ::parser::rsx;
use ::proc_macro::TokenStream;

#[proc_macro]
pub fn rsx(stream: TokenStream) -> TokenStream {
    match rsx::parse(stream.into()) {
        Err(err) => display_rsx_error(err),
        Ok(code) => code.into(),
    }
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, stream: TokenStream) -> TokenStream {
    match component::parse(attr.into(), stream.into()) {
        Err(err) => display_component_error(err),
        Ok(code) => code.into(),
    }
}

fn display_rsx_error(err: rsx::Error) -> TokenStream {
    match err {
        rsx::Error::MismatchedClosingTagCode => {
            panic!("Mismatched closing code, note you can use `</{}>` for simplicity.")
        }
        rsx::Error::MismatchedClosingTagName => panic!("Open and closing tag names don't match"),
        rsx::Error::ExpectedName => {
            panic!("Internal error; expected parsing a name (this should never be visible)")
        }
        rsx::Error::EmptyMacroStreamGiven => panic!("Empty rsx given"),
        rsx::Error::UnexpectedStartingInput => panic!("HTML doesn't start with a node"),
        rsx::Error::UnexpectedToken => panic!("Unexpect token"),
        rsx::Error::ExcessNodesFound => panic!("Excess html found after the initial html"),
        rsx::Error::MoreTokensExpected => {
            panic!("Expected more tokens; could be missing a closing tag?")
        }
        rsx::Error::PeekOnEmptyNode => {
            panic!("Internal error; peeked on an empty node (this should never be visible)")
        }
        rsx::Error::ChompOnEmptyNode => {
            panic!("Internal error; chomped on an empty node (this should never be visible)")
        }
        rsx::Error::FmtError(fmt) => panic!(
            "Internal error; failed writing to string (this should never be visible), {}",
            fmt
        ),
    }
}

fn display_component_error(err: component::Error) -> TokenStream {
    match err {
        component::Error::AttributeFound => panic!(
            "Component macro does not support any attributes; use `#[component]` on it's own."
        ),
        // component::Error::ExpectedName => {
        //     panic!("Internal error; expected parsing a name (this should never be visible)")
        // }
        component::Error::EmptyMacroStreamGiven => panic!("Empty rsx given"),
        // component::Error::UnexpectedStartingInput => panic!("HTML doesn't start with a node"),
        component::Error::ExcessNodesFound => panic!("Excess html found after the initial html"),
        // component::Error::MoreTokensExpected => {
        //     panic!("Expected more tokens; could be missing a closing tag?")
        // }
        // component::Error::PeekOnEmptyNode => {
        //     panic!("Internal error; peeked on an empty node (this should never be visible)")
        // }
        component::Error::UnexpectedToken => panic!("Unexpect token"),
        component::Error::ChompOnEmptyNode => {
            panic!("Internal error; chomped on an empty node (this should never be visible)")
        }
        component::Error::FmtError(fmt) => panic!(
            "Internal error; failed writing to string (this should never be visible), {}",
            fmt
        ),
    }
}
