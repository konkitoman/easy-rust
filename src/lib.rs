use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro_attribute]
pub fn simple(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut item = syn::parse::<syn::Item>(tokens).unwrap();
    match &mut item {
        syn::Item::Struct(item) => {
            let mut defined_lifetimes = Vec::new();
            let mut defined_types = Vec::new();

            for param in item.generics.params.iter() {
                match param {
                    syn::GenericParam::Lifetime(lifetime) => {
                        defined_lifetimes.push(lifetime.lifetime.ident.clone())
                    }
                    syn::GenericParam::Type(type_param) => {
                        defined_types.push(type_param.ident.clone())
                    }
                    _ => {}
                }
            }

            for field in item.fields.iter_mut() {
                match &mut field.ty {
                    syn::Type::Path(path) => {
                        let path = &path.path;
                        if check_generic(path, &defined_types) {
                            defined_types.push(path.segments[0].ident.clone());
                            item.generics
                                .params
                                .push(syn::GenericParam::Type(syn::TypeParam {
                                    attrs: Vec::new(),
                                    ident: path.segments[0].ident.clone(),
                                    colon_token: None,
                                    bounds: Default::default(),
                                    eq_token: None,
                                    default: None,
                                }))
                        }
                    }
                    syn::Type::Reference(ty) => {
                        if let Some(lifetime) = &ty.lifetime {
                            if !defined_lifetimes.contains(&lifetime.ident) {
                                item.generics.params.push(syn::GenericParam::Lifetime(
                                    syn::LifetimeParam {
                                        attrs: Vec::new(),
                                        lifetime: lifetime.clone(),
                                        colon_token: None,
                                        bounds: Default::default(),
                                    },
                                ));
                                defined_lifetimes.push(lifetime.ident.clone());
                            }
                        } else {
                            let mut was_empty = false;
                            if defined_lifetimes.is_empty() {
                                defined_lifetimes.push(Ident::new("a", Span::call_site()));
                                was_empty = true;
                            }
                            let lifetime_ident = defined_lifetimes.first().cloned().unwrap();
                            let lifetime = syn::Lifetime {
                                apostrophe: Span::call_site(),
                                ident: lifetime_ident,
                            };
                            ty.lifetime = Some(lifetime.clone());
                            if was_empty {
                                item.generics.params.push(syn::GenericParam::Lifetime(
                                    syn::LifetimeParam {
                                        attrs: Vec::new(),
                                        lifetime,
                                        colon_token: None,
                                        bounds: Default::default(),
                                    },
                                ))
                            }
                        }

                        if let syn::Type::Path(path) = ty.elem.as_mut() {
                            let path = &path.path;
                            if check_generic(path, &defined_types) {
                                defined_types.push(path.segments[0].ident.clone());
                                item.generics
                                    .params
                                    .push(syn::GenericParam::Type(syn::TypeParam {
                                        attrs: Vec::new(),
                                        ident: path.segments[0].ident.clone(),
                                        colon_token: None,
                                        bounds: Default::default(),
                                        eq_token: None,
                                        default: None,
                                    }))
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            unimplemented!("simple cannot do any thing with: {item:?}")
        }
    }
    quote!(#item).into()
}

fn check_generic(path: &syn::Path, defined_types: &[Ident]) -> bool {
    path.segments.len() == 1
        && !defined_types.contains(&path.segments[0].ident)
        && matches!(
            path.segments[0].ident.to_string().as_str(),
            "A" | "B"
                | "C"
                | "D"
                | "E"
                | "F"
                | "G"
                | "H"
                | "I"
                | "J"
                | "K"
                | "L"
                | "M"
                | "N"
                | "O"
                | "P"
                | "Q"
                | "R"
                | "S"
                | "T"
                | "U"
                | "V"
                | "W"
                | "X"
                | "Y"
                | "Z"
        )
}
