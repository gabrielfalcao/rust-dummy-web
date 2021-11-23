//use std::net::TcpStream;


pub mod http {
    use regex::Regex;
    #[derive(Debug)]
    pub struct Request<'a> {
        pub method: &'a str,
        pub     path: &'a str,
    }
    impl Request<'static> {
        pub fn from_buffer(buffer: &[u8]) -> Option<Request> {
            let line_regex = Regex::new(r"^(GET|POST|PUT|DELETE)\s([^\s]+)\sHTTP/1.1").unwrap();
            let value = std::str::from_utf8(buffer).unwrap();
            return match line_regex.captures(value) {
                Some(cap) => {
                    let method = cap.get(1).unwrap().as_str();
                    let path = cap.get(2).unwrap().as_str();
                    Some(Request { method, path })
                },
                None => None
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use k9::assert_equal;
    #[test]
    fn parses_a_simple_get() {
        let buffer = b"GET / HTTP/1.1\r\n";

        let req = http::Request::from_buffer(buffer).unwrap();

        assert_equal!(req.method, "GET");
        assert_equal!(req.path, "/");

    }
}
