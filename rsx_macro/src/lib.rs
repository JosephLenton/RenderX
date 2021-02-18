use ::proc_macro::TokenStream;
use ::rsx_parser::parse;

#[proc_macro]
pub fn rsx(stream: TokenStream) -> TokenStream {
    parse(stream)
}
