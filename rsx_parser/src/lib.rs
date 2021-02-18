extern crate proc_macro;
use ::proc_macro2::TokenStream;
use ::quote::quote;
use ::quote::format_ident;

pub static BUFFER_NAME : &'static str = "__";

pub fn parse(old_stream: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
  let stream = TokenStream::from(old_stream);
  let buffer_name = format_ident!("{}", BUFFER_NAME);

  let code = quote! {
    let r = {
      #stream
    };

    #buffer_name.render(r);
  };

  code.into()
}
