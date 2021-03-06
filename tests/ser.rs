#![recursion_limit = "512"]
#[macro_use]
extern crate edn_rs;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

    use crate::edn_rs::serialize::Serialize;

    #[test]
    fn serializes_a_complex_structure() {
        ser_struct! {
            #[derive(Debug, Clone)]
            struct Edn {
                btreemap: BTreeMap<String, Vec<String>>,
                btreeset: BTreeSet<i64>,
                hashmap: HashMap<String, Vec<String>>,
                hashset: HashSet<i64>,
                tuples: (i32, bool, char),
            }
        };
        let edn = Edn {
            btreemap: map! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
            btreeset: set! {3i64, 4i64, 5i64},
            hashmap: hmap! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
            hashset: hset! {3i64},
            tuples: (3i32, true, 'd'),
        };

        assert_eq!(edn.serialize(), "{ :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), }");
    }
}

#[test]
fn pub_struct() {
    let edn = helper::Edn {
        val: 6i32,
        tuples: (3i32, true, 'd'),
    };

    assert_eq!(edn.val, 6i32);
    assert_eq!(edn.tuples, (3i32, true, 'd'));
}

mod helper {
    use crate::edn_rs::serialize::Serialize;

    ser_struct! {
        #[derive(Debug, Clone)]
        pub struct Edn {
            val: i32,
            tuples: (i32, bool, char),
        }
    }
}
