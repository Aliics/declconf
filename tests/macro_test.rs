use declconf::Conf;

#[test]
fn test_simple_struct_definition() {
    #[derive(Conf)]
    struct TestConf {
        #[env_var(name = "API_BASE_URL")]
        api_base_url: String,

        #[cli_arg(name = "threads")]
        threads: u16,
    }

    let conf = TestConf::init().expect("should init");

    assert_eq!(conf.api_base_url, "foobar");
    assert_eq!(conf.threads, 8);
}
