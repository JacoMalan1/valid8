use syn::spanned::Spanned;

const EMAIL_REGEX: &str = r#"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#;

#[derive(Clone)]
pub struct FieldValidator {
    pub field: syn::Field,
    pub validator_ident: syn::Ident,
    pub code: proc_macro2::TokenStream,
}

pub fn generate_email_for(
    validator_ident: syn::Ident,
    field: &syn::Field,
    error_ident: &syn::Ident,
) -> FieldValidator {
    let final_ident = quote::format_ident!("{validator_ident}_email");
    if let syn::Type::Path(ref path) = field.ty {
        match path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .as_ref()
        {
            "String" => FieldValidator {
                field: field.clone(),
                validator_ident: final_ident.clone(),
                code: quote::quote! {
                    fn #final_ident(value: &String) -> Result<(), #error_ident> {
                        let reg = ::valid8::regex::RegexBuilder::new(#EMAIL_REGEX).case_insensitive(true).build().unwrap();
                        let validator = ::valid8::validator::Email;
                        validator.validate(value).map_err(|_| #error_ident::Email)
                    }
                },
            },
            _ => FieldValidator {
                field: field.clone(),
                validator_ident: final_ident,
                code: quote::quote_spanned! { field.ty.span() => compile_error!("Type is incompatible with `email` validator."); },
            },
        }
    } else {
        FieldValidator {
            field: field.clone(),
            validator_ident: final_ident,
            code: quote::quote_spanned! { field.span() => compile_error!("`email` validator only applies to path types."); },
        }
    }
}

pub fn generate_required_for(
    validator_ident: syn::Ident,
    field: &syn::Field,
    error_ident: &syn::Ident,
) -> FieldValidator {
    if let syn::Type::Path(ref path) = field.ty {
        match path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .as_ref()
        {
            "String" => {
                let final_validator_ident = quote::format_ident!("{validator_ident}_required");
                let field_name = field.ident.clone().unwrap().to_string();
                let code = quote::quote! {
                    fn #final_validator_ident(value: &String) -> Result<(), #error_ident> {
                        let validator = ::valid8::validator::Required;
                        if validator.validate(value).is_err() {
                            return Err(#error_ident::Missing(#field_name.to_string()));
                        }
                        Ok(())
                    }
                };
                FieldValidator {
                    field: field.clone(),
                    validator_ident: final_validator_ident,
                    code,
                }
            }
            ty => panic!("Type {ty} is incompatible with `required` validator."),
        }
    } else {
        panic!("`required` validator only applies to Path types.")
    }
}
