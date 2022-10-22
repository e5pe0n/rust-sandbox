#[derive(Debug, PartialEq, Eq)]
pub struct RmOptions {
    id: i32,
}

fn rm_parser(mut args: impl Iterator<Item = String>) -> Result<RmOptions, String> {
    let mut id: Option<String> = None;

    // while let Some(option) = args.next() {
    //     while &option[..] {
    //         ""
    //     }
    // }

    Err("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::split_into_string_iter;
    use regex::Regex;
    use rstest::*;

    mod parse_args_tests {
        use super::*;
    }
}
