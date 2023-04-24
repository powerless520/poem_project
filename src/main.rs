use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};
use poem_openapi::OpenApi;

struct Api;

#[OpenApi]
impl Api{
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:8000")).run(app).await
}
