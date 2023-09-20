use reqwest::{StatusCode, Url};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};

use crate::renv::{Package, RenvLock};

const R_VERSION: &str = "4.3";
const REPO: &str = "https://repo.r-wasm.org";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    Done,
    Failed,
}

struct InstallingQueue {
    queue: Arc<Mutex<Vec<(String, std::time::Instant, Status)>>>,
}

impl Clone for InstallingQueue {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
        }
    }
}

struct PackageDownloader {
    package: Package,
    package_url: Url,
    local_path: PathBuf,
    client: reqwest::Client,
}

fn get_package_url(package: &Package) -> String {
    let (package, version) = package.get_package();
    let contrib = format!("{}/bin/emscripten/contrib/{}", REPO, R_VERSION);
    let package_download = format!("{contrib}/{}_{}.tgz", package, version);
    package_download
}

async fn create_download_dir() -> std::path::PathBuf {
    let dir_path = Path::new("./dist/bin/emscripten/contrib/").join(R_VERSION);
    tokio::fs::create_dir_all(&dir_path).await.unwrap();
    dir_path
}

async fn create_package_tar_file(download_path: &Path, package: &str, version: &str) -> File {
    File::create(download_path.join(format!("{}_{}.tgz", package, version)))
        .await
        .unwrap()
}

impl PackageDownloader {
    async fn new(package: &Package, client: reqwest::Client) -> Self {
        let local_path = create_download_dir().await;
        let package_url = get_package_url(package);
        Self {
            package: package.clone(),
            package_url: Url::parse(&package_url).unwrap(),
            local_path,
            client,
        }
    }
    async fn create_tar(&self) -> File {
        let (package, version) = self.package.get_package();
        create_package_tar_file(&self.local_path, package, version).await
    }
    async fn download_package(&self) -> Status {
        let mut res = self
            .client
            .get(self.package_url.clone())
            .send()
            .await
            .unwrap();
        match res.status() {
            StatusCode::OK => {
                // Create a file to stream the body of the response into
                let mut tar = tokio::io::BufWriter::new(self.create_tar().await);
                while let Some(chunk) = res.chunk().await.unwrap() {
                    tar.write_all(&chunk).await.unwrap();
                }
                tar.flush().await.unwrap();
                Status::Done
            }
            _ => Status::Failed,
        }
    }
}

impl Package {
    async fn download(&self, client: reqwest::Client) -> Status {
        let downloader = PackageDownloader::new(self, client).await;
        downloader.download_package().await
    }
}

impl RenvLock {
    pub async fn download(&self) {
        let client = reqwest::Client::new();
        let mut download_tasks = Vec::with_capacity(self.packages().len());
        for package in self.packages() {
            let client = client.clone();
            let package = package.clone();
            download_tasks.push(tokio::spawn(async move {
                (package.download(client).await, package)
            }));
        }
        let results = futures::future::join_all(download_tasks)
            .await
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();
        let mut failed_packages = Vec::new();
        let mut succeeded_packages = Vec::new();
        for (status, package) in results {
            match status {
                Status::Done => succeeded_packages.push(package),
                Status::Failed => failed_packages.push(package),
            }
        }
        eprintln!(
            "Downloaded {} packages successfully: {}",
            succeeded_packages.len(),
            succeeded_packages
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        eprintln!(
            "Failed to download {} packages: {}",
            failed_packages.len(),
            failed_packages
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
