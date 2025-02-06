use proc_macro2::TokenStream;
use syn::{
    FnArg, GenericArgument, Ident, ItemFn, Pat, PatIdent, PatType, PathArguments, Type, TypePath,
};

#[derive(Debug)]
pub struct ParsedAction {
    name: Ident,
    args: Vec<Arg>,
}

impl ParsedAction {
    pub fn new(name: Ident, args: Vec<Arg>) -> Self {
        ParsedAction { name, args }
    }

    pub fn get_name(&self) -> &Ident {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<Arg> {
        &self.args
    }
}

#[derive(Debug)]
pub struct Arg {
    name: Ident,
    ty: Ident,
    is_optional: bool,
}

impl Arg {
    pub fn new(name: Ident, ty: Ident, is_optional: bool) -> Self {
        Arg {
            name,
            ty,
            is_optional,
        }
    }

    pub fn get_name(&self) -> &Ident {
        &self.name
    }

    pub fn get_ty(&self) -> &Ident {
        &self.ty
    }

    pub fn get_is_optional(&self) -> &bool {
        &self.is_optional
    }
}

pub fn parse_action(ast: &ItemFn) -> Result<ParsedAction, syn::Error> {
    //    let ast: ItemFn = syn::parse2(item).unwrap();

    let fn_name = ast.sig.ident.clone();
    let args = extract_args(ast);

    Ok(ParsedAction::new(fn_name, args))
}

fn extract_args(item_fn: &ItemFn) -> Vec<Arg> {
    let mut args = Vec::new();

    for input in item_fn.sig.inputs.iter() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                if let Some((type_ident, is_optional)) = extract_type_ident(&**ty) {
                    args.push(Arg {
                        name: ident.clone(),
                        ty: type_ident,
                        is_optional,
                    });
                }
            }
        }
    }

    args
}

fn extract_type_ident(ty: &Type) -> Option<(Ident, bool)> {
    match ty {
        Type::Reference(type_ref) => extract_type_ident(&*type_ref.elem),
        Type::Path(TypePath { path, .. }) => {
            let segment = path.segments.last()?;
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = angle_bracketed.args.first() {
                        return extract_type_ident(inner_ty)
                            .map(|(inner_ident, _)| (inner_ident, true));
                    }
                }
                None
            } else {
                Some((segment.ident.clone(), false))
            }
        }
        _ => None,
    }
}
