pub mod http {
    //use std::net::TcpStream;
    use regex::Regex;
    use std::fmt;
    use ansi_term::Colour;


    pub struct Request {
        pub method: String,
        pub path: String,
    }

    impl Request {
        pub fn from_buffer(buffer: &[u8]) -> Option<Request> {
            let value = std::str::from_utf8(buffer).unwrap();
            let line_regex = Regex::new(r"^(GET|POST|PUT|DELETE)\s([^\s]+)\sHTTP/1.1").unwrap();
            let cap = line_regex.captures(value)?;
            let method = String::from(cap.get(1)?.as_str());
            let path = String::from(cap.get(2)?.as_str());
            Some(Request { method, path })
        }
    }
    impl fmt::Display for Request {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let method = self.method.as_str();
            let method = match method {
                "GET" => Colour::Green.bold().paint(method),
                "POST" => Colour::Yellow.bold().paint(method),
                "DELETE" => Colour::Red.bold().paint(method),
                "PUT" => Colour::Blue.bold().paint(method),
                "PATCH" => Colour::Cyan.bold().paint(method),
                _ => Colour::White.bold().paint(method),
            };
            write!(f, "Request(method={}, path={})", method, self.path)
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
