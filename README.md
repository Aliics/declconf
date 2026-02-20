# declconf

*Decl*arative *Conf*iguration for Rust.
Heavily leverages `proc_macro_derive` and macros to make building configuration objects simple.

```rs
#[derive(Conf)]
struct TestConf {
    #[env_var("API_BASE_URL")]
    api_base_url: String,

    #[cli_arg("threads")]
    #[ignored]
    threads: u16,
}

// Then usage is very simple
TestConf::init()?;
```

## Features

- Aggregate errors for configuration issues.
- Environment Variable and CLI arg support.
- Compile-time errors for bad setup.
- `FromStr` support for config types.

## Future Features

- Nesting
- `Option` support
- Defaults
- JSON/TOML loading
