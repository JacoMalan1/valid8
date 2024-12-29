const EMAIL_REGEX: &str = r#"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#;

/// Trait for an object that can validate an item of type `T`
pub trait Validator<T> {
    /// The error returned if validation fails.
    type Error;

    /// Validates `value`.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Self::Error`] if validation fails, [`Result::Ok`] otherwise.
    fn validate(&self, value: &T) -> Result<(), Self::Error>;
}

/// Ensures that the field is a valid email.
#[derive(Debug, Copy, Clone)]
pub struct Email;

impl<T> Validator<T> for Email
where
    T: AsRef<str>,
{
    type Error = ();

    fn validate(&self, value: &T) -> Result<(), Self::Error> {
        let re = regex::RegexBuilder::new(EMAIL_REGEX)
            .case_insensitive(false)
            .build()
            .unwrap();

        if re.is_match(value.as_ref()) {
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Ensures that a value is 'not empty'.
///
/// E.g. for strings, the value has more than one non-whitespace character.
#[derive(Debug, Copy, Clone)]
pub struct Required;

impl Validator<&str> for Required {
    type Error = ();

    fn validate(&self, value: &&str) -> Result<(), Self::Error> {
        if value.trim().is_empty() {
            Err(())
        } else {
            Ok(())
        }
    }
}

impl Validator<String> for Required {
    type Error = ();

    fn validate(&self, value: &String) -> Result<(), Self::Error> {
        if value.trim().is_empty() {
            Err(())
        } else {
            Ok(())
        }
    }
}

impl<T> Validator<Option<T>> for Required {
    type Error = ();

    fn validate(&self, value: &Option<T>) -> Result<(), ()> {
        if value.is_some() {
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Ensures that a value is bounded from below by some other value.
/// E.g. For strings, ensures that the length of the string is greater than some minimum.
/// For integer types, ensures the value is greater than some minimum.
#[derive(Debug, Copy, Clone)]
pub struct Min<T>(T);

impl<T> Min<T>
where
    T: Ord,
{
    /// Constructs a new [`Min`] validator.
    pub fn new(min: T) -> Self {
        Self(min)
    }
}

macro_rules! min_impl {
    ( $x:ty ) => {
        impl Validator<$x> for Min<$x> {
            type Error = ();
            fn validate(&self, value: &$x) -> Result<(), Self::Error> {
                if *value >= self.0 {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    };
}

min_impl!(u8);
min_impl!(i8);
min_impl!(u16);
min_impl!(i16);
min_impl!(u32);
min_impl!(i32);
min_impl!(u64);
min_impl!(i64);
min_impl!(usize);
min_impl!(isize);

macro_rules! min_impl_str {
    ( $x:ty ) => {
        impl Validator<&str> for Min<$x> {
            type Error = ();
            fn validate(&self, value: &&str) -> Result<(), Self::Error> {
                if value.len() >= usize::try_from(self.0).expect("Value couldn't fit into usize.") {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }

        impl Validator<String> for Min<$x> {
            type Error = ();
            fn validate(&self, value: &String) -> Result<(), Self::Error> {
                if value.len() >= usize::try_from(self.0).expect("Value couldn't fit into usize.") {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    };
}

min_impl_str!(u8);
min_impl_str!(u16);
min_impl_str!(u32);
min_impl_str!(u64);
min_impl_str!(usize);
