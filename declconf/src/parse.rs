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
