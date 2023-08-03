use hyper::service::{make_service_fn, service_fn};
use hyper::{server::Server, Body, Request, Response};
use server::{contains_valid_characters, get_json_from_table_async};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    // Table prefix handler
    if path.starts_with("/table/") {
        let table_name = path
            .strip_prefix("/table/")
            .expect("Error with getting path suffix");

        // Проверка на отсутствие символов, кроме букв, цифр и нижнего подчёркивания
        if !contains_valid_characters(table_name) {
            return Ok(Response::new(Body::from("Wrong table name format")));
        }

        // let json_data = get_json_from_table(table_name);
        let json_data = get_json_from_table_async(table_name);
        let mut response = Response::new(Body::from(json_data));
        let headers = response.headers_mut();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        return Ok(response);
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
