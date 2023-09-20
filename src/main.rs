use webR_bundler::renv::RenvLock;

#[tokio::main]
async fn main() {
    let renv_lock = std::fs::File::open("renv.lock").unwrap();
    let renv_lock = serde_json::from_reader::<_, RenvLock>(renv_lock).unwrap();
    renv_lock.download().await;
}
