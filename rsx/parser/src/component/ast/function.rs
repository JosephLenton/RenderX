use ::proc_macro2::Group;
use ::proc_macro2::Ident;

#[derive(Clone, Debug)]
pub struct Function {
    pub public: Option<Public>,
    pub name: Ident,
    pub generics: Option<Generics>,
    pub params: Group,
    pub where_clause: Option<WhereClause>,
    pub code: Group,
}

#[derive(Clone, Debug)]
pub struct Public {}

#[derive(Clone, Debug)]
pub struct WhereClause {}

#[derive(Clone, Debug)]
pub struct Generics {}

// impl PartialEq for Attribute {
//     fn eq(&self, other: &Self) -> bool {
//         self.key == other.key && self.value == other.value
//     }
// }
