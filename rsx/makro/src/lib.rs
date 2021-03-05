use ::parser::parse;
use ::parser::Error;
use ::proc_macro::TokenStream;

#[proc_macro]
pub fn rsx(stream: TokenStream) -> TokenStream {
    match parse(stream.into()) {
        Err(err) => display_error(err),
        Ok(code) => code.into(),
    }
}

fn display_error(err: Error) -> TokenStream {
    match err {
        Error::ExpectedName => panic!("Internal error; expected parsing a name (this should never be visible)"),
        Error::EmptyMacroStreamGiven => panic!("Empty rsx given"),
        Error::UnexpectedStartingInput => panic!("HTML doesn't start with a node"),
        Error::UnexpectedToken => panic!("Unexpect token"),
        Error::ExcessNodesFound => panic!("Excess html found after the initial html"),
        Error::MismatchedTagName => panic!("Open and closing tag names don't match"),
        Error::MoreTokensExpected => panic!("Expected more code"),
        Error::PeekOnEmptyNode => {
            panic!("Internal error; peeked on an empty node (this should never be visible)")
        }
        Error::ChompOnEmptyNode => {
            panic!("Internal error; chomped on an empty node (this should never be visible)")
        }
        Error::FmtError(fmt) => panic!(
            "Internal error; failed writing to string (this should never be visible), {}",
            fmt
        ),
    }
}
