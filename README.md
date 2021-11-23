# Dummy Web Server in Rust

> Tested in Rust v1.56

Teaching myself rust by writting a small HTTP server.

Based on the [Chapter 20: "Final Project: Building a Multithreaded Web
Server"](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
of the Rust book, but I added a few other things like the ability to
embed static files in the final binary.

The idea is that the final binary will have a full website.


### Done so far:

- Single-threaded server
- Parse HTTP requests
- Hardcoded routing mapping known paths (currently only "/")
- Serve static files for paths that are not mapped

### TODO:

- Multi-threaded server
- Route registration
- Dynamic routing


## Running


```bash
cargo run
```

## Testing


```bash
cargo test
```
