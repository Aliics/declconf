# declconf

*Decl*arative *Conf*iguration for Rust.
Heavily leverages `proc_macro_derive` and macros to make building configuration objects simple.

```rs
use declconf::Conf;

// For the FromStr impl for AuthProvider.
use std::str::FromStr;

#[derive(Conf)]
struct AppConf {
    #[env_var("API_BASE_URL")]
    api_base_url: String,

    #[env_var("JWT_JWKS_URL")]
    jwt_jwks_url: String,

    #[env_var("OVERRIDE_AUTH_PROVIDER")]
    override_auth_provider: Option<AuthProvider>,

    #[cli_arg("max-threads")]
    max_threads: u16,
}

#[derive(Clone)]
enum AuthProvider {
    Custom,
    ZeroAuth,
    Cognito,
}

// To be serializable as as a Conf struct, we need to impl FromStr.
impl FromStr for AuthProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AuthProvider::*;
        match s {
            "custom" => Ok(Custom),
            "zeroauth" => Ok(ZeroAuth),
            "cognito" => Ok(Cognito),
            otherwise => Err(format!("Unknown provider: {otherwise}")),
        }
    }
}

// Initialize your app and go!
AppConf::init();
```

## Features

- Aggregate errors for configuration issues.
- Environment Variable and CLI arg support.
- Compile-time errors for bad setup.
- `FromStr` support for config types.
- Optional fields
