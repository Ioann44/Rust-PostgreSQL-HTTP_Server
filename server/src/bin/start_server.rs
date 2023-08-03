use hyper::service::{make_service_fn, service_fn};
use hyper::{server::Server, Body, Request, Response};
use server::establish_connection;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    // Table prefix handler
    if path.starts_with("/table/") {
        let table_name = path
            .strip_prefix("/table/")
            .expect("Error with getting path suffix")
            .to_string();

        let connection = &mut establish_connection();

        if false {
            let mut response = Response::new(Body::from(""));
            let headers = response.headers_mut();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            return Ok(response);
        }

        return Ok(Response::new(Body::from(
            "Where is no table with with name",
        )));
    }

    // 404 error
    let response = Response::builder()
        .status(404)
        .body(Body::from("404 Not Found"))
        .unwrap();
    Ok(response)
}

#[tokio::main]
async fn main() {
    dotenvy::from_path(".env").ok();
    let port = env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set")
        .parse::<u16>()
        .unwrap();

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);
    println!("Server started on port {port}");

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
