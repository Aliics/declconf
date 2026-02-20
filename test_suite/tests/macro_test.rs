use std::env;

use declconf::Conf;

#[test]
fn test_simple_struct_definition() {
    #[derive(Conf)]
    struct TestConf {
        #[env_var("API_BASE_URL")]
        api_base_url: String,

        #[cli_arg("threads")]
        #[ignored]
        threads: u16,
    }

    unsafe {
        env::set_var("API_BASE_URL", "http://localhost/");
    }

    let conf = TestConf::init().expect("should init");

    assert_eq!(conf.api_base_url, "http://localhost/");
    assert_eq!(conf.threads, 0);
}
