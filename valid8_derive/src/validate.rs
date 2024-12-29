use syn::spanned::Spanned;

const EMAIL_REGEX: &str = r#"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#;

#[derive(Clone)]
pub struct FieldValidator {
    pub field: syn::Field,
    pub validator_ident: syn::Ident,
    pub code: proc_macro2::TokenStream,
}

pub fn generate_email_for(validator_ident: syn::Ident, field: &syn::Field) -> FieldValidator {
    let final_ident = quote::format_ident!("{validator_ident}_email");
    FieldValidator {
        field: field.clone(),
        validator_ident: final_ident.clone(),
        code: quote::quote_spanned! { field.span() =>
            fn #final_ident(value: &str) -> Result<(), ::valid8::ValidationError> {
                let reg = ::valid8::regex::RegexBuilder::new(#EMAIL_REGEX).case_insensitive(true).build().unwrap();
                let validator = ::valid8::validator::Email;
                validator.validate(&value).map_err(|_| ::valid8::ValidationError::Email)
            }
        },
    }
}

pub fn generate_required_for(validator_ident: syn::Ident, field: &syn::Field) -> FieldValidator {
    let final_validator_ident = quote::format_ident!("{validator_ident}_required");
    let field_name = field.ident.clone().unwrap().to_string();
    let code = quote::quote_spanned! { field.span() =>
        fn #final_validator_ident(value: &str) -> Result<(), ::valid8::ValidationError> {
            let validator = ::valid8::validator::Required;
            if validator.validate(&value).is_err() {
                return Err(::valid8::ValidationError::Missing(#field_name.to_string()));
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

pub fn generate_min_for<T: quote::ToTokens>(
    min: T,
    validator_ident: syn::Ident,
    field: &syn::Field,
) -> FieldValidator {
    let final_validator_ident = quote::format_ident!("{validator_ident}_min");
    let field_name = field.ident.clone().unwrap().to_string();
    let code = quote::quote_spanned! { field.span() =>
        fn #final_validator_ident<'a, T>(value: &'a T) -> Result<(), ::valid8::ValidationError>
        where
            ::valid8::validator::Min<u32>: ::valid8::Validator<T>
        {
            let validator = ::valid8::validator::Min::<u32>::new(#min);
            if validator.validate(&value).is_err() {
                return Err(::valid8::ValidationError::Min(#field_name.to_string()));
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
