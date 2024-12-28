#![warn(
    missing_docs,
    rust_2018_idioms,
    clippy::pedantic,
    missing_debug_implementations
)]

//! # Valid8
//! A simple validation library.
//!
//! # Example
//!
//! ```rust
//! use valid8::Validator;
//! use valid8::validator::Min;
//!
//! let validator = Min::<u32>::new(5);
//! let invalid = "1234";
//! assert!(validator.validate(invalid).is_err());
//!
//! let valid = "12345";
//! assert!(validator.validate(valid).is_ok());
//! ```

/// Validators for different types.
pub mod validator;

pub use regex;
pub use validator::Validator;

#[cfg(feature = "derive")]
pub use valid8_derive::Validate;
