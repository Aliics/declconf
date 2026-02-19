use std::{error::Error, fmt::Display, str::FromStr};

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
        todo!()
    }
}

impl Error for ConfError {}

pub fn from_env_var<F, E>(name: &str) -> Result<F, ConfError>
where
    F: FromStr<Err = E> + Clone,
    E: ToString,
{
    std::env::var(name)
        .map_err(|_| {
            ConfError::MissingField(MissingField {
                field_name: name.to_string(),
            })
        })?
        .parse()
        .map_err(|e: E| {
            ConfError::ParseError(ParseError {
                field_name: name.to_string(),
                message: e.to_string(),
            })
        })
}
