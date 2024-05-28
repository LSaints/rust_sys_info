use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;

    println!("Server on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
    })
    .bind((host, port))?
    .run()
    .await
}
