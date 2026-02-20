use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Lit, Token, parse_macro_input,
    punctuated::Punctuated,
};

/// Create compile error in a quote to make more obvious errors.
/// Panicking does not provide good information.
macro_rules! gen_compile_error {
    ($msg:expr) => {
        quote! {
            compile_error!($msg)
        }
    };
}

#[proc_macro_derive(Conf, attributes(env_var, cli_arg, ignored))]
/// Struct is parsed and is expanded to have an `init` function available.
pub fn conf_derivation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data) => match data.fields {
            // We only allow this on named structs.
            Fields::Named(ref fields) => gen_from_fields(input.ident, fields.clone()),
            Fields::Unnamed(_) => {
                gen_compile_error!("#[derive(Conf)] not allowed on tuple structs")
            }
            _ => gen_compile_error!("#[derive(Conf)] not allowed on unit structs"),
        },
        _ => gen_compile_error!("#[derive(Conf)] must be used on a struct"),
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
                    gen_compile_error!("env_var is used multiple times on struct field")
                } else if cli_arg_attrs.len() > 1 {
                    gen_compile_error!("cli_arg is used multiple times on struct field")
                } else {
                    gen_field_fetch_for_attrs(field_name, env_var_attrs, cli_arg_attrs)
                }
            };

            (field_name, fetch)
        })
        .unzip();

    quote! {
        impl #struct_name {
            pub fn init() -> Result<Self, ::declconf::ConfErrors> {
                let arg_map = ::declconf::build_arg_map();

                #(#field_fetchs;)*

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
    field_name: &Option<Ident>,
    env_var_attrs: Vec<&Attribute>,
    cli_arg_attrs: Vec<&Attribute>,
) -> proc_macro2::TokenStream {
    match (env_var_attrs.first(), cli_arg_attrs.first()) {
        (None, None) => gen_compile_error!("env_var or cli_arg must be present on struct fields"),
        (Some(_), Some(_)) => {
            gen_compile_error!("env_var and cli_arg cannot both be present on struct fields")
        }
        // #[cli_arg("my-cli-arg")]
        (None, Some(cli_arg)) => match parse_attr_arg_name(cli_arg) {
            Some(field_name_str) => quote! {
                let #field_name = ::declconf::from_cli_arg(arg_map, #field_name_str)
            },
            None => gen_compile_error!("Invalid cli_arg(NAME) meta attribute on struct field"),
        },
        // #[env_var("MY_ENV_VAR")]
        (Some(env_var), None) => match parse_attr_arg_name(env_var) {
            Some(field_name_str) => quote! {
                let #field_name = ::declconf::from_env_var(#field_name_str)
            },
            None => gen_compile_error!("Invalid env_var(NAME) meta attribute on struct field"),
        },
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
        .parse_args_with(Punctuated::<Lit, Token![,]>::parse_terminated)
        .ok()?;

    for lit in lit_args {
        match lit {
            // Let's just accept strings for now.
            // We may come back and add more config options.
            Lit::Str(s) => return Some(s.value()),
            _ => return None,
        }
    }

    None
}

fn token_to_string<T: quote::ToTokens>(t: &T) -> String {
    quote!(#t).to_string()
}
