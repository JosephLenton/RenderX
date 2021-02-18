use ::proc_macro2::Ident;
use ::proc_macro2::Span;

pub struct CommonIdents {
  pub LeftAngle : Ident,
  pub RightAngle : Ident,
  pub ForwardSlash : Ident,
}

impl CommonIdents {
  pub fn new() -> Self {
    Self {
      LeftAngle : Ident::new("<", Span::call_site()),
      RightAngle: Ident::new(">", Span::call_site()),
      ForwardSlash : Ident::new("/", Span::call_site()),
    }
  }
}
