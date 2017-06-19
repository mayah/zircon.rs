use std::collections::HashMap;
use url;

/// Query parameter or Form parameter.
pub struct Query {
    map: HashMap<String, Vec<String>>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            map: HashMap::new(),
        }
    }

    /// Parses query.
    pub fn from_string(s: &str) -> Query {
        let p = url::form_urlencoded::parse(s.as_bytes());

        let mut map = HashMap::<String, Vec<String>>::new();
        for (key, value) in p {
            map.entry(key.to_string()).or_insert(Vec::new()).push(value.to_string());
        }

        Query {
            map: map,
        }
    }

    /// Returns the first parameter. If key doesn't exist, None is returned.
    pub fn find_first(&self, key: &str) -> Option<&str> {
        self.map.get(key).and_then(|vs| {
            if vs.is_empty() {
                None
            } else {
                Some(vs[0].as_ref())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parse_query_empty() {
        let q = Query::from_string("");
        let expected = HashMap::new();
        assert_eq!(q.map, expected);
    }

    #[test]
    fn parse_query_single() {
        let q = Query::from_string("a=b");

        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["b".to_string()]);

        assert_eq!(q.map, expected);
        assert_eq!(q.find_first("a"), Some("b"));
        assert_eq!(q.find_first("b"), None);
    }

    #[test]
    fn parse_query_simple() {
        let q = Query::from_string("a=b&c=d&e=f");

        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["b".to_string()]);
        expected.insert("c".to_string(), vec!["d".to_string()]);
        expected.insert("e".to_string(), vec!["f".to_string()]);

        assert_eq!(q.map, expected);
        assert_eq!(q.find_first("a"), Some("b"));
        assert_eq!(q.find_first("c"), Some("d"));
        assert_eq!(q.find_first("e"), Some("f"));
        assert_eq!(q.find_first("b"), None);
        assert_eq!(q.find_first("d"), None);
        assert_eq!(q.find_first("f"), None);
    }

    #[test]
    fn decode_string_plus() {
        let q = Query::from_string("a=b+c");

        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["b c".to_string()]);
        assert_eq!(q.map, expected);
    }

    #[test]
    fn decode_string_multi() {
        let q = Query::from_string("a=b&a=c");

        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["b".to_string(), "c".to_string()]);
        assert_eq!(q.map, expected);
    }

    #[test]
    fn parse_query_edge_case() {
        let q = Query::from_string("a=b=c&d");

        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["b=c".to_string()]);
        expected.insert("d".to_string(), vec!["".to_string()]);

        assert_eq!(q.map, expected);
        assert_eq!(q.find_first("a"), Some("b=c"));
        assert_eq!(q.find_first("b"), None);
        assert_eq!(q.find_first("c"), None);
        assert_eq!(q.find_first("d"), Some(""));
        assert_eq!(q.find_first("e"), None);
        assert_eq!(q.find_first("f"), None);
    }

    #[test]
    fn decode_string_edge_case() {
        let q = Query::from_string("a=Thyme%20%26time%3Dagain");

        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["Thyme &time=again".to_string()]);
        assert_eq!(q.map, expected);
    }
}
