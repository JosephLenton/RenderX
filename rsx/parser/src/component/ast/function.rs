use ::syn::token::Async;
use ::syn::token::Const;
use ::syn::token::Unsafe;
use ::syn::Attribute;
use ::syn::Block;
use ::syn::Ident;
use ::syn::Pat;
use ::syn::Type;
use ::syn::Visibility;

#[derive(Clone, Debug)]
pub struct Function {
    pub visibility: Visibility,
    pub constness: Option<Const>,
    pub asyncness: Option<Async>,
    pub unsafety: Option<Unsafe>,
    pub name: Ident,
    pub return_type: Box<Type>,
    pub props: Option<Props>,
    pub code: Box<Block>,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub attributes: Vec<Attribute>,
    pub pattern: Box<Pat>,
    pub item_type: Box<Type>,
}
