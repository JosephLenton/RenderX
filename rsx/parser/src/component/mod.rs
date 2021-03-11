mod ast;
mod error;
mod grammar;
mod output;

pub use self::error::*;

use ::proc_macro2::TokenStream;

pub fn parse(old_attrs: TokenStream, old_stream: TokenStream) -> Result<TokenStream> {
    if !old_attrs.is_empty() {
        return Err(Error::AttributeFound);
    }

    let stream = TokenStream::from(old_stream);
    let ast = grammar::parse(stream)?;
    Ok(output::build(ast))
}

#[cfg(test)]
mod parse {
    use super::*;
    use ::quote::quote;

    #[test]
    fn it_should_output_component_markup() -> Result<()> {
        let output = parse(
            quote! {},
            quote! {
                fn MyBanner(props: MyBannerProps) -> Node {
                    rsx! {
                        <div class="my-banner">
                            <h1>My Banner</h1>
                        </div>
                    }
                }
            },
        )?;

        let expected = quote! {
            #![allow(non_snake_case)]
            fn MyBanner(props: MyBannerProps) -> Node {
                rsx! {
                    <div class="my-banner">
                        <h1>My Banner</h1>
                    </div>
                }
            }
        };

        assert_tokens_eq(expected, output)
    }

    fn assert_tokens_eq(expected: TokenStream, output: TokenStream) -> Result<()> {
        ::pretty_assertions::assert_eq!(expected.to_string(), output.to_string());

        Ok(())
    }
}
