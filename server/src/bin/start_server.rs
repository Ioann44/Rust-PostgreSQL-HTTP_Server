use hyper::service::{make_service_fn, service_fn};
use hyper::{server::Server, Body, Request, Response};
use server::get_json_from_table_async;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    // Регулярное выражение корректного запроса
    let re = regex::Regex::new(r"^/table/([A-Za-z0-9_]+)/?$").unwrap();
    if let Some(captures) = re.captures(path) {
        if let Some(table_name) = captures.get(1) {
            // Получение результата запроса с bool значением "успеха"
            let (json_data, result_received) = get_json_from_table_async(table_name.as_str());
            if result_received {
                // Отправка результата
                let response = Response::builder()
                    .header("Content-Type", "application/json")
                    .body(Body::from(json_data))
                    .unwrap();
                return Ok(response);
            }
        }
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
    let server = Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(shutdown_signal()); // Have no effect if send request on localhost instead of 127.0.0.1
    println!("Server started on port {port}");

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
