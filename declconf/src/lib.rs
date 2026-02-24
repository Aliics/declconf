//! Declaratively setup configuration for your program neatly into
//! simple structs by annotating them their sources. Catch common issues
//! at compile time and wire up your program with a single function call.
//!
//! Fields can be declared to capture environment variable or
//! command-line arguments.
//!
//! # Simple configuration example
//! ```
//! use declconf::Conf;
//!
//! // For the FromStr impl for AuthProvider.
//! use std::str::FromStr;
//!
//! #[derive(Conf)]
//! struct AppConf {
//!     #[env_var("API_BASE_URL")]
//!     api_base_url: String,
//!
//!     #[env_var("JWT_JWKS_URL")]
//!     jwt_jwks_url: String,
//!
//!     #[env_var("OVERRIDE_AUTH_PROVIDER")]
//!     override_auth_provider: Option<AuthProvider>,
//!
//!     #[cli_arg("max-threads")]
//!     max_threads: u16,
//! }
//!
//! #[derive(Clone)]
//! enum AuthProvider {
//!     Custom,
//!     ZeroAuth,
//!     Cognito,
//! }
//!
//! // To be serializable as as a Conf struct, we need to impl FromStr.
//! impl FromStr for AuthProvider {
//!     type Err = String;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         use AuthProvider::*;
//!         match s {
//!             "custom" => Ok(Custom),
//!             "zeroauth" => Ok(ZeroAuth),
//!             "cognito" => Ok(Cognito),
//!             otherwise => Err(format!("Unknown provider: {otherwise}")),
//!         }
//!     }
//! }
//!
//! // Initialize your app and go!
//! AppConf::init();
//! ```

pub mod args;
pub mod error;
pub mod parse;

pub use args::*;
pub use declconf_derive::Conf;
pub use error::*;
pub use parse::*;
