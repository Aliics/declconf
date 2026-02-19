use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, parse_macro_input};

macro_rules! gen_compile_error {
    ($msg:expr) => {
        quote! {
            compile_error!($msg);
        }
        .into()
    };
}

#[proc_macro_derive(Conf, attributes(env_var, cli_arg, ignore))]
pub fn conf_derivation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => gen_from_fields_named(struct_name, fields.clone()),
            Fields::Unnamed(ref fields) => unimplemented!(), //gen_from_fields_unnamed(struct_name, fields.clone()),
            _ => gen_compile_error!("#[derive(Conf)] not allowed on tuple or union structs"),
        },
        _ => gen_compile_error!("#[derive(Conf)] must be used on a struct"),
    }
}

fn gen_from_fields_named(struct_name: Ident, fields: FieldsNamed) -> TokenStream {
    let recurse_fields = fields.named.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: std::env::var("#name")?.parse().unwrap()
        }
    });

    quote! {
        impl #struct_name {
            pub fn init() -> Result<Self, std::env::VarError> {
                Ok(Self {
                    #(#recurse_fields,)*
                })
            }
        }
    }
    .into()
}

fn gen_from_fields_unnamed(struct_name: Ident, fields: FieldsUnnamed) -> TokenStream {
    quote! {}.into()
}

