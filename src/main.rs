use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rust_embed::RustEmbed;
use web_server::http::Request;
use ansi_term::Colour::Red;
use ansi_term::Colour::Blue;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

#[derive(RustEmbed)]
#[folder = "public/"]
#[include = "*.html"]
struct Asset;

fn main() {
    let port = 3000;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!(
        "{} {}",
        Yellow.bold().paint("Web Server listening on"),
        Blue.bold().underline().paint(format!("http://{}", address))
    );
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn load_html(filename: &str) -> Option<String> {
    let file = Asset::get(filename);
    match file {
        Some(file) => {
            return unsafe {
                let value = std::str::from_utf8_unchecked(file.data.as_ref());
                Some(value.to_string())
            };
        }
        None => None,
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let request = Request::from_buffer(&buffer).unwrap();

    let (status_line, filename) = match request.path {
        "/" => ("HTTP/1.1 200 OK", "index.html"),
        _ => (
            "HTTP/1.1 200 OK",
            request.path.strip_prefix("/").unwrap_or(request.path),
        ),
    };

    let response = match load_html(&filename) {
        Some(contents) => {
            println!(
                "Request {} {} 200 OK",
                Green.bold().paint(request.method),
                Green.paint(request.path)
            );

            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            )
        }
        None => {
            println!(
                "Request {} {} 404 Not Found",
                Red.bold().paint(request.method),
                Red.paint(request.path)
            );
            let contents = load_html("404.html").unwrap();
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                "HTTP/1.1 404 Not Found",
                contents.len(),
                contents
            )
        }
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
