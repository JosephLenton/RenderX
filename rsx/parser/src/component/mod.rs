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
    fn it_should_output_component_markup_without_props() -> Result<()> {
        let output = parse(
            quote! {},
            quote! {
                pub fn HorizontalRule() -> Node {
                    rsx! {
                        <hr class="horizontal-rule" />
                    }
                }
            },
        )?;

        let expected = quote! {
            #[allow(non_snake_case)]
            pub struct HorizontalRule;

            impl FnOnce<()> for HorizontalRule {
                type Output = Node;
                extern "rust-call" fn call_once(self, _: ()) -> Node {
                    rsx! {
                        <hr class="horizontal-rule" />
                    }
                }
            }

            impl ::renderx::Component for HorizontalRule {
                type Props = ();
            }
        };

        assert_tokens_eq(expected, output)
    }

    #[test]
    fn it_should_output_component_markup_with_props() -> Result<()> {
        let output = parse(
            quote! {},
            quote! {
                pub fn MyBanner(my_props: MyBannerProps) -> Node {
                    rsx! {
                        <div class="my-banner">
                            <h1>My Banner</h1>
                        </div>
                    }
                }
            },
        )?;

        let expected = quote! {
            #[allow(non_snake_case)]
            pub struct MyBanner;

            impl FnOnce<(MyBannerProps,)> for MyBanner {
                type Output = Node;
                extern "rust-call" fn call_once(self, (my_props,): (MyBannerProps,)) -> Node {
                    rsx! {
                        <div class="my-banner">
                            <h1>My Banner</h1>
                        </div>
                    }
                }
            }

            impl ::renderx::Component for MyBanner {
                type Props = MyBannerProps;
            }
        };

        assert_tokens_eq(expected, output)
    }

    fn assert_tokens_eq(expected: TokenStream, output: TokenStream) -> Result<()> {
        ::pretty_assertions::assert_eq!(expected.to_string(), output.to_string());

        Ok(())
    }
}
