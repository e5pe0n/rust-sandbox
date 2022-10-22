use crate::todo::TodoList;

#[derive(Debug, PartialEq, Eq)]
pub struct AddOptions {
    title: String,
    description: String,
    label: String,
}

pub fn add_parser(mut args: impl Iterator<Item = String>) -> Result<AddOptions, String> {
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut label: Option<String> = None;

    while let Some(option) = args.next() {
        match &option[..] {
            "-t" | "--title" => {
                title = args.next();
            }
            "-d" | "--description" => {
                description = args.next();
            }
            "-l" | "--label" => {
                label = args.next();
            }
            _ => return Err(format!("invalid option; {}", option)),
        }
    }

    Ok(AddOptions {
        title: match title {
            Some(x) => x,
            None => return Err(String::from("-t option required")),
        },
        description: description.unwrap_or_default(),
        label: label.unwrap_or_default(),
    })
}

pub fn add(options: AddOptions) -> Result<(), String> {
    // TODO: impl
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::split_into_string_iter;
    use regex::Regex;
    use rstest::*;

    fn add_options1() -> AddOptions {
        AddOptions {
            title: "title1".to_string(),
            description: "description1".to_string(),
            label: "label1".to_string(),
        }
    }
    mod parse_args_tests {
        use super::*;

        #[rstest]
        #[case(
            split_into_string_iter("-t title1 -d description1 -l label1"),
            Ok(add_options1())
        )]
        #[case(
            split_into_string_iter("-t title1"),
            Ok(AddOptions {
                title: "title1".to_string(),
                description: "".to_string(),
                label: "".to_string(),
            })
        )]
        #[case(
            split_into_string_iter("--title title1 --description description1 --label label1"),
            Ok(add_options1())
        )]
        #[case(
            split_into_string_iter("-t title1 --description description1 -l label1"),
            Ok(add_options1())
        )]
        fn should_parse_args(
            #[case] args: impl Iterator<Item = String>,
            #[case] expected: Result<AddOptions, String>,
        ) {
            assert_eq!(add_parser(args), expected,)
        }

        #[rstest]
        #[case(
            "-d description1 -l label1".split(' ').map(String::from),
        )]
        fn should_return_error_if_required_option_missing(
            #[case] args: impl Iterator<Item = String>,
        ) {
            let re = Regex::new(r"-t option required").unwrap();

            let parse_res = add_parser(args);
            assert!(parse_res.is_err(), "not err; parse_res={:?}", parse_res);

            let message = &parse_res.unwrap_err();
            assert!(re.is_match(message), "message={message}");
        }

        #[rstest]
        #[case(split_into_string_iter("-t title1 -x invalid_option"))]
        fn should_return_error_if_invalid_option_given(#[case] args: impl Iterator<Item = String>) {
            let re = Regex::new(r"invalid option").unwrap();

            let parse_res = add_parser(args);
            assert!(parse_res.is_err(), "not err; parse_res={:?}", parse_res);

            let message = &parse_res.unwrap_err();
            assert!(re.is_match(message), "message={message}");
        }
    }
}
