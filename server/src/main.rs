mod routes;
mod models;

use actix_web::{App, HttpServer};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;

    println!("Server on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
        .service(os::return_os_info)
        .service(memory::return_memory_info)
    })
    .bind((host, port))?
    .run()
    .await
}
