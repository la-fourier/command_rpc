use syn::{parse_quote, FnArg, PatType, ReturnType};

pub fn output_check(rt: &ReturnType) {
    if let syn::ReturnType::Type(_, boxed) = rt {
        if let syn::Type::Path(path) = *boxed.clone() {
            if let Some(ident) = path.path.get_ident() {
                // Some unpacking of the whole Structure (TODO merch maybe)
                // Checks if the return type is a String or a str or implements the ToStr trait
                if ident.to_string() == "String" {
                    eprintln!(
                        "Your cli returns a String. This is ok but might cause speed issues."
                    );
                } else if ident.to_string() == "str" {
                    eprintln!("Your cli returns a str, fine!");
                } else {
                    if path
                        .path
                        .segments
                        .iter()
                        .any(|s| s.ident.to_string() == String::from("std::str::ToStr"))
                    {
                        eprintln!("Your cli returns a type that does implement ToStr so this is a valid output.");
                    } else {
                        eprintln!("Your cli returns a type that does not implement ToStr so this is a invalid output.");
                        // TODO feature: own default parser
                    }
                }
            }
        }
    }
}

pub fn input_check(arg: &FnArg) -> std::io::Result<()> {
    if let FnArg::Typed(PatType {
        attrs: _,
        pat: _,
        colon_token: _,
        ty,
    }) = arg
    {
        if match *ty.clone() {
            syn::Type::Path(types) => {
                if let Some(segment) = types.path.segments.last() {
                    segment.ident.to_string() != String::from("std::str::FromStr")
                } else {
                    true
                }
            }
            syn::Type::Slice(types) => {
                if let syn::Type::Path(type_path) = *types.clone().elem {
                    if let Some(segment) = type_path.path.segments.last() {
                        segment.ident.to_string() != String::from("std::str::FromStr")
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            syn::Type::Tuple(types) => types.elems.iter().any(|elem| {
                if let Err(_err) = input_check(&FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(parse_quote!(_)),
                    colon_token: parse_quote!(:),
                    ty: Box::new(elem.clone()),
                })) {
                    true
                } else {
                    false
                }
            }),
            syn::Type::BareFn(_) => {
                eprintln!("Your cli cannot take a function because you can´t give code to your cli at runtime.");
                false
            }
            syn::Type::Group(_) => {
                eprintln!("Your cli cannot take a group because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::Paren(_) => {
                eprintln!("Your cli cannot take a paren because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::Reference(_) => {
                eprintln!("Your cli cannot take a reference because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::TraitObject(_) => {
                eprintln!("Your cli cannot take a trait object because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::ImplTrait(_) => {
                eprintln!("Your cli cannot take a impl trait because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::Infer(_) => {
                eprintln!("Your cli cannot take a infer because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::Macro(_) => {
                eprintln!("Your cli cannot take a macro because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::Never(_) => {
                eprintln!("Your cli cannot take a never because else this proc macro would have to check all types and it should be easy for you to find an other solution.");
                false
            }
            syn::Type::Array(_) => false,
            _ => {
                eprintln!("This is not finished yet, please be careful with your input types. ");
                false
            }
        } {
            // TODO feature: own default parser
            eprintln!("Your cli takes a type that does not implement FromStr but the crpc macro needs it to parse the arguments. Please implement FromStr for your type.");
        };
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "This is not finished yet, please be careful with your input types. ",
    ))
    // Ok(())
}
