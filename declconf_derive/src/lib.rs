use std::fmt;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/// Declare configuration sources and initialization functions generated.
///
/// Struct is parsed and is expanded to have initialization functions.
/// All fields on the structure should have an `env_var` or `cli_arg`
/// attribute, otherwise an `ignore` attribute should be present to
/// let the macro know that know parsing should happen.
///
/// # Examples
///
/// ```ignore
/// use crate::Conf;
///
/// #[derive(Conf)]
/// struct TestConf {
///     #[env_var("API_BASE_URL")]
///     api_base_url: String,
///
///     #[cli_arg("threads")]
///     #[ignored]
///     threads: u16,
///
///     #[cli_arg("extras")]
///     extras: Option<String>,
/// }
/// ```
#[proc_macro_derive(Conf, attributes(env_var, cli_arg, ignored))]
pub fn conf_derivation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data) => match data.fields {
            // We only allow this on named structs.
            Fields::Named(ref fields) => gen_from_fields(input.ident, fields.clone()),
            Fields::Unnamed(_) => {
                gen_compile_error(input, "#[derive(Conf)] not allowed on tuple structs")
            }
            _ => gen_compile_error(input, "#[derive(Conf)] not allowed on unit structs"),
        },
        _ => gen_compile_error(input, "#[derive(Conf)] must be used on a struct"),
    }
    .into()
}

fn gen_from_fields(struct_name: Ident, fields: FieldsNamed) -> proc_macro2::TokenStream {
    let (field_names, field_fetchs): (Vec<_>, Vec<_>) = fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let ty = &field.ty;

            let fetch = if !find_matching_field_attrs(field, "ignored").is_empty() {
                // We don't want to do further proc, we assume this is just entirely ignored.
                // A default does need to exist for the type though.
                quote! {
                    let #field_name = Ok::<#ty, ::declconf::ConfError>(Default::default());
                }
            } else {
                let env_var_attrs = find_matching_field_attrs(field, "env_var");
                let cli_arg_attrs = find_matching_field_attrs(field, "cli_arg");

                if env_var_attrs.len() > 1 {
                    gen_compile_error(
                        env_var_attrs.last(),
                        "env_var is used multiple times on struct field",
                    )
                } else if cli_arg_attrs.len() > 1 {
                    gen_compile_error(
                        cli_arg_attrs.last(),
                        "cli_arg is used multiple times on struct field",
                    )
                } else {
                    gen_field_fetch_for_attrs(
                        field,
                        env_var_attrs.first().cloned(),
                        cli_arg_attrs.first().cloned(),
                    )
                }
            };

            (field_name, fetch)
        })
        .unzip();

    quote! {
        impl #struct_name {
            pub fn init() -> Result<Self, ::declconf::ConfErrors> {
                // Accumulate our arguments in advance.
                // Otherwise we need to parse them on every pass.
                let arg_map = ::declconf::build_arg_map();

                Self::init_with_arg_map(arg_map)
            }

            pub fn init_with_arg_map(arg_map: ::declconf::ArgMap) -> Result<Self, ::declconf::ConfErrors> {
                #(#field_fetchs;)*

                // Aggregate all errors here.
                // We get to emit all failures this way.
                let mut errs = ::declconf::ConfErrors(vec![]);
                #({
                    if let Err(e) = #field_names.clone() {
                        errs.0.push(e);
                    }
                })*

                if !errs.0.is_empty() {
                    Err(errs)
                } else {
                    Ok(Self {
                        #(#field_names: #field_names.unwrap(),)*
                    })
                }
            }
        }
    }
}

fn gen_field_fetch_for_attrs(
    field: &Field,
    env_var_attr: Option<&Attribute>,
    cli_arg_attr: Option<&Attribute>,
) -> proc_macro2::TokenStream {
    match (env_var_attr, cli_arg_attr) {
        (None, None) => gen_compile_error(
            &field.ident,
            "env_var or cli_arg must be present on struct fields",
        ),
        (Some(_), Some(_)) => gen_compile_error(
            &field.ident,
            "env_var and cli_arg cannot both be present on struct fields",
        ),
        // #[cli_arg("my-cli-arg")]
        (None, Some(cli_arg)) => gen_field_fetch(
            field,
            cli_arg,
            |field_name, field_name_str| {
                quote! {
                    let #field_name = ::declconf::from_cli_arg(arg_map, #field_name_str)
                }
            },
            |field_name, field_name_str| {
                quote! {
                    let #field_name = ::declconf::optional_from_cli_arg(arg_map, #field_name_str)
                }
            },
        ),
        // #[env_var("MY_ENV_VAR")]
        (Some(env_var), None) => gen_field_fetch(
            field,
            env_var,
            |field_name, field_name_str| {
                quote! {
                    let #field_name = ::declconf::from_env_var(#field_name_str)
                }
            },
            |field_name, field_name_str| {
                quote! {
                    let #field_name = ::declconf::optional_from_env_var(#field_name_str)
                }
            },
        ),
    }
}

fn gen_field_fetch(
    field: &Field,
    env_var: &Attribute,
    extract_fn: fn(&Option<Ident>, String) -> proc_macro2::TokenStream,
    optional_extract_fn: fn(&Option<Ident>, String) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let is_option_ty = token_to_string(&field.ty).starts_with("Option ");

    match parse_attr_arg_name(env_var) {
        None => gen_compile_error(&field.ident, "Missing name argument from attribute"),
        Some(field_name_str) if field_name_str.is_empty() => {
            gen_compile_error(env_var, "Name argument cannot be an empty string")
        }
        Some(field_name_str) if is_option_ty => optional_extract_fn(&field.ident, field_name_str),
        Some(field_name_str) => extract_fn(&field.ident, field_name_str),
    }
}

fn find_matching_field_attrs<'a>(field: &'a Field, name: &'a str) -> Vec<&'a Attribute> {
    field
        .attrs
        .iter()
        .filter(|a| token_to_string(a.path()) == name)
        .collect()
}

fn parse_attr_arg_name(attr: &Attribute) -> Option<String> {
    let lit_args = attr
        .parse_args_with(punctuated::Punctuated::<Lit, Token![,]>::parse_terminated)
        .ok()?;

    for lit in lit_args {
        match lit {
            // Let's just accept strings for now.
            // We may come back and add more config options.
            Lit::Str(s) => return Some(s.value()),
            _ => return None,
        }
    }

    // Fallback for nothing.
    None
}

fn gen_compile_error<T, M>(token: T, message: M) -> proc_macro2::TokenStream
where
    T: quote::ToTokens,
    M: fmt::Display,
{
    Error::new_spanned(token, message).to_compile_error()
}

fn token_to_string<T: quote::ToTokens>(t: &T) -> String {
    quote!(#t).to_string()
}
