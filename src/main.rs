#[macro_use]
extern crate serde_derive;
use actix_web::{guard, middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use returning::*;
use routing::*;

mod error;
mod returning;
mod routing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    println!("Server starting at http://{}:{}/", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // Routing Examples
            .service(hello_service)
            .route("hello_route", web::get().to(hello_route))
            .service(
                web::resource("/manual_service")
                    .route(web::route().guard(guard::Get()).to(manual_service)),
            )
            // Returning Examples
            .service(custom_responder)
            .service(json_responder)
            .service(manual_response)
            .service(custom_error)
    })
    .bind((host, port))?
    .run()
    .await
}
