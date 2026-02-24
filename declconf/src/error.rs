//! Configuration errors that can occur during runtime.
//!
//! Centered around the `ConfErrors` type which is an aggregate of
//! errors that occured when constructing the configuration data.

use std::{error::Error, fmt::Display};

/// Aggregate of configuration errors that occurred when building
/// the configuration data. This will be accumulated so that when
/// programs start up, all the configuration that is missing or
/// malformed is all given up front. Prevents the usual flow of
/// configuration whack-a-mole.
#[derive(Clone, Debug)]
pub struct ConfErrors(pub Vec<ConfError>);

impl Display for ConfErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", self.0)
    }
}

impl Error for ConfErrors {}

#[derive(Clone, Debug)]
pub struct MissingField {
    pub field_name: String,
}

#[derive(Clone, Debug)]
pub struct ParseError {
    pub field_name: String,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum ConfError {
    MissingField(MissingField),
    ParseError(ParseError),
}

impl Display for ConfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ConfError::*;
        match self {
            MissingField(missing_field) => write!(f, "MissingField: {}", missing_field.field_name),
            ParseError(parse_error) => write!(
                f,
                "ParseError: Failed to parse {}: {}",
                parse_error.field_name, parse_error.message
            ),
        }
    }
}

impl Error for ConfError {}
