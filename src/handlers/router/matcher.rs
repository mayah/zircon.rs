use regex::{Captures, Regex};

static VAR_SEQ: &'static str = "[,.a-zA-Z0-9_-]*";
static VAR_SEQ_WITH_SLASH: &'static str = "[,./a-zA-Z0-9_-]*";

pub struct RouteResult {
    params: Vec<(String, String)>,
}

impl RouteResult {
    pub fn param(&self, key: &str) -> Option<&str> {
        for &(ref k, ref v) in &self.params {
            if k == key {
                return Some(&v);
            }
        }

        return None;
    }
}

pub struct Matcher {
    regex: Regex,
}

impl Matcher {
    pub fn new(regex: Regex) -> Matcher {
        Matcher {
            regex: regex,
        }
    }

    pub fn match_route(&self, path: &str) -> Option<RouteResult> {
        if !self.regex.is_match(path) {
            return None;
        }

        if let Some(captures) = self.regex.captures(path) {
            let mut params = Vec::new();

            for (opt_name, opt_value) in self.regex.capture_names().zip(captures.iter()) {
                if let (Some(name), Some(value)) = (opt_name, opt_value) {
                    params.push((name.to_string(), value.as_str().to_string()));
                }
            }

            return Some(RouteResult {
                params: params,
            });
        };

        return None;
    }
}

impl<'a> From<&'a str> for Matcher {
    fn from(s: &'a str) -> Matcher {
        From::from(s.to_string())
    }
}

impl From<String> for Matcher {
    fn from(s: String) -> Matcher {
        let regex_var_seq: Regex = Regex::new(r":([,a-zA-Z0-9_-]*)").unwrap();

        let with_placeholder = s.replace("**", "__DOUBLEWILDCARD__");
        let star_replaced = with_placeholder.replace("*", VAR_SEQ);
        let wildcarded = star_replaced.replace("__DOUBLEWILDCARD__", VAR_SEQ_WITH_SLASH);

        let named_captures = regex_var_seq.replace_all(&wildcarded, |captures: &Captures| {
            let c = captures.iter().skip(1).next().unwrap();
            // println!("c={}", c.unwrap().as_str());
            format!("(?P<{}>[,a-zA-Z0-9%_-]*)", c.unwrap().as_str())
        });

        let regex_str = format!("^{}$", named_captures.to_string());

        // println!("regex={}", regex_str);
        let regex = Regex::new(&regex_str).unwrap();
        Matcher::new(regex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path_match_1() {
        let matcher: Matcher = "/".into();

        assert!(matcher.match_route("/").is_some());

        assert!(matcher.match_route("/foo").is_none());

        // Basically we shouldn't pass query_string to match_route.
        assert!(matcher.match_route("/?foo").is_none());
        assert!(matcher.match_route("/?foo=bar").is_none());
    }

    #[test]
    fn test_simple_path_match_2() {
        let matcher: Matcher = "/foo/bar".into();

        assert!(matcher.match_route("/foo/bar").is_some());

        assert!(matcher.match_route("/").is_none());
        assert!(matcher.match_route("/f").is_none());
        assert!(matcher.match_route("/foo").is_none());
        assert!(matcher.match_route("/foo/bar/").is_none());
        assert!(matcher.match_route("/foo/bar/baz").is_none());
    }

    #[test]
    fn test_param_path_match_1() {
        let matcher: Matcher = "/foo/:id".into();

        assert!(matcher.match_route("/").is_none());
        assert!(matcher.match_route("/f").is_none());
        assert!(matcher.match_route("/foo").is_none());
        assert!(matcher.match_route("/foo/bar/").is_none());
        assert!(matcher.match_route("/foo/bar/baz").is_none());

        let rr = matcher.match_route("/foo/bar").unwrap();
        assert_eq!(rr.param("id"), Some("bar"));
        assert_eq!(rr.param("foo"), None);
        assert_eq!(rr.param("piyo"), None);
    }

    #[test]
    fn test_param_path_match_2() {
        let matcher: Matcher = "/:user/:issue".into();

        assert!(matcher.match_route("/").is_none());
        assert!(matcher.match_route("/f").is_none());
        assert!(matcher.match_route("/foo").is_none());
        assert!(matcher.match_route("/foo/bar/").is_none());
        assert!(matcher.match_route("/foo/bar/baz").is_none());

        let rr = matcher.match_route("/foo/bar").unwrap();
        assert_eq!(rr.param("user"), Some("foo"));
        assert_eq!(rr.param("issue"), Some("bar"));
        assert_eq!(rr.param("foo"), None);
        assert_eq!(rr.param("piyo"), None);
    }

    #[test]
    fn test_param_with_star() {
        let matcher: Matcher = "/*/:issue".into();

        assert!(matcher.match_route("/").is_none());
        assert!(matcher.match_route("/f").is_none());
        assert!(matcher.match_route("/foo").is_none());
        assert!(matcher.match_route("/foo/bar/").is_none());
        assert!(matcher.match_route("/foo/bar/baz").is_none());

        {
            let rr = matcher.match_route("/foo/bar").unwrap();
            assert_eq!(rr.param("user"), None);
            assert_eq!(rr.param("issue"), Some("bar"));
            assert_eq!(rr.param("foo"), None);
            assert_eq!(rr.param("piyo"), None);
        }
        {
            let rr = matcher.match_route("/foobar/bar").unwrap();
            assert_eq!(rr.param("user"), None);
            assert_eq!(rr.param("issue"), Some("bar"));
            assert_eq!(rr.param("foo"), None);
            assert_eq!(rr.param("piyo"), None);
        }
    }

    #[test]
    fn test_param_with_doublestar() {
        let matcher: Matcher = "/:issue/**".into();

        assert!(matcher.match_route("/").is_none());
        assert!(matcher.match_route("/f").is_none());
        assert!(matcher.match_route("/foo").is_none());

        assert!(matcher.match_route("/foo/bar/").is_some());
        assert!(matcher.match_route("/foo/bar/baz").is_some());

        {
            let rr = matcher.match_route("/foo/bar").unwrap();
            assert_eq!(rr.param("issue"), Some("foo"));
            assert_eq!(rr.param("foo"), None);
            assert_eq!(rr.param("piyo"), None);
        }
        {
            let rr = matcher.match_route("/foobar/bar").unwrap();
            assert_eq!(rr.param("issue"), Some("foobar"));
            assert_eq!(rr.param("foo"), None);
            assert_eq!(rr.param("piyo"), None);
        }
        {
            let rr = matcher.match_route("/foobar/bar/baz/piyo").unwrap();
            assert_eq!(rr.param("issue"), Some("foobar"));
            assert_eq!(rr.param("foo"), None);
            assert_eq!(rr.param("piyo"), None);
        }
    }

    #[test]
    fn test_param_with_doublestar2() {
        let matcher: Matcher = "/public/**".into();

        assert!(matcher.match_route("/public/foo/bar.css").is_some());
    }
}
