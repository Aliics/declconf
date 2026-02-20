use std::{collections::HashMap, env};

pub type ArgMap = HashMap<String, Option<String>>;

pub fn build_arg_map() -> ArgMap {
    build_arg_map_from_vec(env::args().collect())
}

fn build_arg_map_from_vec(args: Vec<String>) -> ArgMap {
    let mut arg_map = ArgMap::new();
    for (i, arg) in args.iter().cloned().enumerate() {
        if has_arg_prefix(&arg) {
            arg_map.insert(
                arg[2..].to_string(),
                // Just yoink the next, but also check that it isn't an arg.
                args.iter()
                    .cloned()
                    .nth(i + 1)
                    .filter(|s| !has_arg_prefix(s))
                    .clone(),
            );
        }
    }
    arg_map
}

fn has_arg_prefix(s: &String) -> bool {
    s.starts_with("--")
}

#[cfg(test)]
mod test {
    use crate::args::build_arg_map_from_vec;

    #[test]
    fn should_build_from_cli_args() {
        let arg_map = build_arg_map_from_vec(
            vec![
                "prog_name",
                "--foo",
                "bar",
                "--flag",
                "--another-flag",
                "--n",
                "64",
            ]
            .iter()
            .map(ToString::to_string)
            .collect(),
        );

        assert_eq!(arg_map.get("foo").unwrap().clone().unwrap(), "bar");
        assert_eq!(arg_map.get("flag").unwrap().clone(), None);
        assert_eq!(arg_map.get("another-flag").unwrap().clone(), None);
        assert_eq!(arg_map.get("n").unwrap().clone().unwrap(), "64");
    }
}
