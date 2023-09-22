use std::path::PathBuf;

use actix_files as fs;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn server(ourdir: PathBuf, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/", ourdir.clone()).index_file("index.html"))
            .wrap(Logger::default().log_target("webr::server"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
