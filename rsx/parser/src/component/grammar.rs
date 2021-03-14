use crate::component::ast::Function;
use crate::component::ast::Props;
use crate::component::error::Error;
use crate::component::error::Result;

use ::proc_macro2::TokenStream;

use ::syn::parse2;
use ::syn::FnArg;
use ::syn::ItemFn;
use ::syn::ReturnType;

use ::syn::punctuated::Pair;
use ::syn::punctuated::Punctuated;
use ::syn::token::Comma;

pub fn parse(stream: TokenStream) -> Result<Function> {
    if stream.is_empty() {
        return Err(Error::EmptyMacroStreamGiven);
    }

    let f = parse2::<ItemFn>(stream)?;
    let signature = f.sig;

    let return_type = match signature.output {
        ReturnType::Default => {
            return Err(Error::NoReturnType);
        }
        ReturnType::Type(_, r_type) => r_type,
    };

    Ok(Function {
        visibility: f.vis,
        constness: signature.constness,
        asyncness: signature.asyncness,
        unsafety: signature.unsafety,
        name: signature.ident,
        props: parse_props(signature.inputs)?,
        return_type,
        code: f.block,
    })
}

fn parse_props(mut input: Punctuated<FnArg, Comma>) -> Result<Option<Props>> {
    if input.is_empty() {
        return Ok(None);
    }

    if input.len() > 1 {
        return Err(Error::ExtraParametersFound);
    }

    let fn_arg = match input.pop() {
        Some(Pair::End(fn_arg)) => fn_arg,
        _ => {
            unreachable!("Seen multiple parameters, when there should only be one at this time (this is a bug).")
        }
    };

    match fn_arg {
        FnArg::Receiver(_) => {
            return Err(Error::SelfArgUnsupported);
        }
        FnArg::Typed(pat_type) => Ok(Some(Props {
            attributes: pat_type.attrs,
            pattern: pat_type.pat,
            item_type: pat_type.ty,
        })),
    }
}
