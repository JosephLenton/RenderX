use ::proc_macro2::Group;
use ::proc_macro2::Ident;
use ::proc_macro2::TokenStream;

#[derive(Clone, Debug)]
pub struct Function {
    pub public: Option<Public>,
    pub name: Ident,
    pub generics: Option<Generics>,
    pub params: Params,
    pub rest: TokenStream,
}

#[derive(Clone, Debug)]
pub struct Public {}

#[derive(Clone, Debug)]
pub struct Generics {}

#[derive(Clone, Debug)]
pub struct Params {
    pub tokens: Group,
}

// impl PartialEq for Attribute {
//     fn eq(&self, other: &Self) -> bool {
//         self.key == other.key && self.value == other.value
//     }
// }
