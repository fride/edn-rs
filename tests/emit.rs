extern crate edn_rs;

#[cfg(test)]
mod tests {
    use crate::edn_rs::json_to_edn;

    #[test]
    fn emits_helloworld_edn() {
        let json = String::from("{\"hello\": \"world\"}");
        let edn = String::from("{:hello \"world\"}");

        assert_eq!(edn, json_to_edn(json));
    }

    #[test]
    fn replaces_whitespace_in_keys_by_dash() {
        let json = String::from("{\"hello world\": \"julia\"}");
        let edn = String::from("{:hello-world \"julia\"}");

        assert_eq!(edn, json_to_edn(json));
    }

    #[test]
    fn emits_nil_edn() {
        let json = String::from("{\"hello\": null}");
        let edn = String::from("{:hello nil}");

        assert_eq!(edn, json_to_edn(json));
    }

    #[test]
    fn emits_char_edn() {
        let json = String::from("{\"hello\": 'c'}");
        let edn = String::from("{:hello \\c}");

        assert_eq!(edn, json_to_edn(json));
    }

    #[test]
    fn emits_number_edn() {
        let json = String::from("{\"multi_string with underscore\": 545643}");
        let edn = String::from("{:multi-string-with-underscore 545643}");

        assert_eq!(edn, json_to_edn(json));
    }

    #[test]
    fn multiline_json_to_edn() {
        let json = String::from(
            "{
            \"hello\": [
                {
                    \"country name\": \"brazil\",
                    \"word\": \"mundo\"
                },
                {
                    \"country name\": \"usa\",
                    \"word\": \"world\"
                }
            ]
        }",
        );
        let edn = String::from(
            "{
            :hello [
                {
                    :country-name \"brazil\",
                    :word \"mundo\"
                },
                {
                    :country-name \"usa\",
                    :word \"world\"
                }
            ]
        }",
        );

        assert_eq!(edn, json_to_edn(json));
    }
}
