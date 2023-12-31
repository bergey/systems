use std::io;
use tiny_http::{Server, Response};

fn main() -> Result<(), io::Error> {
    let server = Server::http("0.0.0.0:7878").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
        );

        let response = Response::from_string("hello world");
        let _ = request.respond(response);
    }
    Ok(())
}
