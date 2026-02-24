use std::{env, str::FromStr};

use declconf::{ArgMap, Conf};

#[test]
fn test_simple_struct_definition() {
    #[derive(Conf)]
    struct TestConf {
        #[env_var("API_BASE_URL")]
        api_base_url: String,

        #[env_var("REGION_OVERRIDE")]
        region_override: Option<Region>,

        #[env_var("REGION_FALLBACK")]
        region_fallback: Region,

        #[cli_arg("threads")]
        #[ignored]
        threads: u16,

        #[cli_arg("extras")]
        extras: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq)]
    enum Region {
        UsEast1,
        UsEast2,
        UsWest1,
        UsWest2,
        ApSoutheast1,
        ApSoutheast2,
    }

    impl FromStr for Region {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use Region::*;
            match s {
                "us-east-1" => Ok(UsEast1),
                "us-east-2" => Ok(UsEast2),
                "us-west-1" => Ok(UsWest1),
                "us-west-2" => Ok(UsWest2),
                "ap-southeast-1" => Ok(ApSoutheast1),
                "ap-southeast-2" => Ok(ApSoutheast2),
                otherwise => Err(format!("Unknown string {otherwise}")),
            }
        }
    }

    unsafe {
        env::set_var("API_BASE_URL", "http://localhost/");
        env::set_var("REGION_FALLBACK", "us-east-1");
    }

    let mut arg_map = ArgMap::new();
    arg_map.insert("extras".to_string(), Some("fast-mode,less-mem".to_string()));

    let conf = TestConf::init_with_arg_map(arg_map).expect("should init");

    assert_eq!(conf.api_base_url, "http://localhost/");
    assert!(conf.region_override.is_none());
    assert_eq!(conf.region_fallback, Region::UsEast1);
    assert_eq!(conf.threads, 0);
    assert_eq!(conf.extras.unwrap(), "fast-mode,less-mem")
}
