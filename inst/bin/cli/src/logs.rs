use env_logger::{Builder, Env};
use log::LevelFilter;

pub fn init() {
    Builder::from_env(Env::new().filter_or("WEBR_LOG", "info"))
        .filter_module("actix_server", LevelFilter::Off)
        .init()
}
