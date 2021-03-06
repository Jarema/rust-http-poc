//! A Hello World example application for working with Gotham.

extern crate gotham;

use gotham::state::State;

const HELLO_WORLD: &str = "Hello World!";

struct Config {
    port: i32
}

impl Config {
    fn new(port: &i32) -> Config {
        Config { port: port.clone() }
    }
}

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let conf = Config::new(&7878);
    let addr = format!("127.0.0.1:{}", conf.port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::{hyper::StatusCode, test::TestServer};

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}