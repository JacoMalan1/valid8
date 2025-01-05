use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

mod validate;

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident,
        data: syn::Data::Struct(syn::DataStruct { fields, .. }),
        ..
    } = syn::parse::<syn::DeriveInput>(input).unwrap()
    else {
        panic!("Can only derive Validate on structs.")
    };

    let field_validator_results = fields
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            field
                .clone()
                .attrs
                .into_iter()
                .filter_map(|syn::Attribute { meta, .. }| match meta {
                    syn::Meta::List(list) => {
                        if list.path.get_ident().map(|i| i.to_string())
                            == Some("validate".to_string())
                        {
                            Some(
                                list.parse_args_with(
                                    Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                                )
                                .expect("Invalid syntax"),
                            )
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .flat_map(|punct| punct.into_iter())
                .map(|arg| -> Result<validate::FieldValidator, TokenStream> {
                    let field_ident = field
                        .ident
                        .clone()
                        .map(|x| x.to_string())
                        .unwrap_or(idx.to_string());

                    let validator_base_ident =
                        quote::format_ident!("__validate_{}_field_{field_ident}", ident);

                    match &arg {
                        syn::Meta::Path(p) => {
                            if p.is_ident("required") {
                                Ok(validate::generate_required_for(validator_base_ident, &field))
                            } else if p.is_ident("email") {
                                Ok(validate::generate_email_for(validator_base_ident, &field))
                            } else {
                                Err(quote::quote_spanned! { arg.span() => compile_error!("Unknown validator"); }.into())
                            }
                        },
                        syn::Meta::List(syn::MetaList { path, tokens, .. }) => {
                            if path.is_ident("min") {
                                let lit = syn::parse::<syn::LitInt>(tokens.clone().into()).expect("Invalid expression.");
                                Ok(validate::generate_min_for(lit, validator_base_ident, &field))
                            } else {
                                Err(quote::quote_spanned! { arg.span() => compile_error!("Unknown validator"); }.into())
                            }
                        }
                        _ => Err(quote::quote_spanned! { arg.span() =>
                            compile_error!("Invalid validator");
                        }
                        .into()),
                    }
                })
                .collect::<Vec<_>>()
        })
        .flat_map(|x| x.into_iter())
        .map(|res| {
            res.map(
                |validate::FieldValidator {
                     field,
                     validator_ident,
                     code,
                 }| (field, validator_ident, code),
            )
        })
        .collect::<Vec<_>>();

    let mut field_validators = vec![];
    for res in field_validator_results {
        match res {
            Ok(x) => field_validators.push(x),
            Err(tokens) => return tokens,
        }
    }

    let field_validators_code = field_validators.iter().map(|x| x.2.clone());
    let validation_code = field_validators.iter().map(|(field, ident, _)| {
        let field_ident = field.ident.clone().unwrap();
        quote::quote! {
            #ident(&self.#field_ident)?;
        }
    });

    quote::quote! {
        #[automatically_derived]
        #[allow(dead_code, unused_variables)]
        impl #ident {
            pub fn validate(&self) -> Result<(), ::valid8::ValidationError> {
                use ::valid8::Validator;
                #(
                    #[allow(
                        non_snake_case,
                        clippy::ptr_arg,
                        clippy::unwrap_used,
                        clippy::ignored_unit_patterns,
                        clippy::needless_lifetimes,
                        clippy::needless_borrow,
                    )]
                    #field_validators_code
                )*
                #(#validation_code)*
                Ok(())
            }
        }
    }
    .into()
}
