use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
#[include = "*.html"]
struct Asset;

fn main() {
    let port = 3000;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Web Server listening on {}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn load_html(filename: &str) -> String {
    let file = Asset::get(filename).expect(format!("file not found: {}", filename).as_str());
    let result = std::str::from_utf8(file.data.as_ref()).expect("failed to load index.html as utf-8");
    return String::from(result);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let  get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = load_html(&filename);
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
