use ::std::convert::From;

pub type Result<N> = ::std::result::Result<N, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NoReturnType,
    ExtraParametersFound,
    SelfArgUnsupported,
    AttributeFound,
    EmptyMacroStreamGiven,
    SynError(syn::parse::Error),
}

impl From<syn::parse::Error> for Error {
    fn from(err: syn::parse::Error) -> Self {
        Error::SynError(err)
    }
}
