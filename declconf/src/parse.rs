//! Data source fetching and parsing.
//!
//! These functions won't generally be used directly, and instead these
//! are for use by the `declconf_derive` crate, and the code that gets
//! generated from using the `Conf` derive macro.

use std::{env, str::FromStr};

use crate::{
    ArgMap,
    error::{ConfError, MissingField, ParseError},
};

pub fn from_env_var<F, E>(name: &str) -> Result<F, ConfError>
where
    F: FromStr<Err = E> + Clone,
    E: ToString,
{
    env::var(name)
        .map_err(|_| {
            ConfError::MissingField(MissingField {
                field_name: name.to_string(),
            })
        })?
        .parse()
        .map_err(|e: E| err_to_parse_err::<F, E>(name, e))
}

pub fn optional_from_env_var<F, E>(name: &str) -> Result<Option<F>, ConfError>
where
    F: FromStr<Err = E> + Clone,
    E: ToString,
{
    match from_env_var(name) {
        Ok(v) => Ok(Some(v)),
        Err(ConfError::MissingField(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn from_cli_arg<F, E>(arg_map: ArgMap, name: &str) -> Result<F, ConfError>
where
    F: FromStr<Err = E> + Clone,
    E: ToString,
{
    match arg_map.get(name) {
        Some(Some(value)) => value
            .parse()
            .map_err(|e: E| err_to_parse_err::<F, E>(name, e)),
        _ => Err(ConfError::MissingField(MissingField {
            field_name: name.to_string(),
        })),
    }
}

pub fn optional_from_cli_arg<F, E>(arg_map: ArgMap, name: &str) -> Result<Option<F>, ConfError>
where
    F: FromStr<Err = E> + Clone,
    E: ToString,
{
    match arg_map.get(name) {
        Some(Some(value)) => value
            .parse()
            .map(Some)
            .map_err(|e: E| err_to_parse_err::<F, E>(name, e)),
        _ => Ok(None),
    }
}

fn err_to_parse_err<F, E>(name: &str, e: E) -> ConfError
where
    F: FromStr<Err = E> + Clone,
    E: ToString,
{
    ConfError::ParseError(ParseError {
        field_name: name.to_string(),
        message: e.to_string(),
    })
}
