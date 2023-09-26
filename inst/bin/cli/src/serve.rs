use crate::errors::{BundlerResult, Error};
use actix_files as fs;
use actix_web::{middleware::Logger, App, HttpServer};
use std::path::{Path, PathBuf};

fn check_if_outdir_exists(outdir: impl AsRef<Path>) -> BundlerResult<()> {
    if outdir.as_ref().try_exists()? {
        Ok(())
    } else {
        Err(Error::NoDistDir(outdir.as_ref().into()))
    }
}

pub async fn server(outdir: PathBuf, port: u16) -> BundlerResult<()> {
    check_if_outdir_exists(&outdir)?;
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/", outdir.clone()).index_file("index.html"))
            .wrap(Logger::default().log_target("webr::server"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;
    Ok(())
}
