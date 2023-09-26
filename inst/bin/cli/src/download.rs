use crate::errors::BundlerResult;
use colored::Colorize;

use reqwest::{StatusCode, Url};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Semaphore;
use tokio::time::Instant;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};

use crate::renv::{Package, RenvLock};
use crate::repo::VesionMatcher;

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

async fn create_download_dir(outdir: impl AsRef<Path>) -> BundlerResult<PathBuf> {
    let dir_path = outdir
        .as_ref()
        .join("repo")
        .join("bin")
        .join("emscripten")
        .join("contrib")
        .join(R_VERSION);
    tokio::fs::create_dir_all(&dir_path).await?;
    Ok(dir_path)
}

async fn create_package_tar_file(
    download_path: &Path,
    package: &str,
    version: &str,
) -> BundlerResult<File> {
    let file = File::create(download_path.join(format!("{}_{}.tgz", package, version))).await?;
    Ok(file)
}

impl PackageDownloader {
    async fn new(
        package: &Package,
        outdir: impl AsRef<Path>,
        client: reqwest::Client,
    ) -> BundlerResult<Self> {
        let local_path = create_download_dir(outdir.as_ref()).await?;
        let package_url = get_package_url(package);
        Ok(Self {
            package: package.clone(),
            package_url: Url::parse(&package_url)?,
            local_path,
            client,
        })
    }
    async fn create_tar(&self) -> BundlerResult<File> {
        let (package, version) = self.package.get_package();
        create_package_tar_file(&self.local_path, package, version).await
    }
    async fn download_package(&self) -> BundlerResult<Status> {
        let mut res = self.client.get(self.package_url.clone()).send().await?;
        match res.status() {
            StatusCode::OK => {
                // Create a file to stream the body of the response into
                let mut tar = tokio::io::BufWriter::new(self.create_tar().await?);
                while let Some(chunk) = res.chunk().await? {
                    tar.write_all(&chunk).await?;
                }
                tar.flush().await?;
                Ok(Status::Done)
            }
            _ => Ok(Status::Failed),
        }
    }
}

impl Package {
    async fn download(
        &self,
        outdir: impl AsRef<Path>,
        client: reqwest::Client,
    ) -> BundlerResult<Status> {
        let instant = std::time::Instant::now();
        let downloader = PackageDownloader::new(self, outdir, client).await?;
        let status = downloader.download_package().await?;
        eprintln!(
            "Downloaded {} in {}",
            self.to_string().green(),
            format!("{:.0?}", instant.elapsed()).cyan().italic()
        );
        Ok(status)
    }
}

impl RenvLock {
    pub async fn download(
        &mut self,
        outdir: impl AsRef<Path>,
        parallel_downloads: usize,
    ) -> BundlerResult<()> {
        let outdir: Arc<Path> = Arc::from(outdir.as_ref());
        let client = reqwest::Client::new();
        let version_matcher = VesionMatcher::new(client.clone()).await?;
        version_matcher.sync_renv(self);
        let mut download_tasks = Vec::with_capacity(self.packages().len());
        let semaphore = Arc::new(Semaphore::new(parallel_downloads));
        let start_time = Instant::now();
        for package in self.packages() {
            let client = client.clone();
            let package = package.clone();
            let semaphore = Arc::clone(&semaphore);
            let outdir = Arc::clone(&outdir);
            download_tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.expect("Semaphore is closed");
                BundlerResult::Ok((package.download(outdir, client).await?, package))
            }));
        }
        let results = futures::future::join_all(download_tasks)
            .await
            .into_iter()
            .map(|result| result.expect("Failed to join download tasks"))
            .collect::<Vec<_>>();
        let mut failed_packages = Vec::new();
        let mut succeeded_packages = Vec::new();
        for result in results {
            let (status, package) = result?;
            match status {
                Status::Done => succeeded_packages.push(package),
                Status::Failed => failed_packages.push(package),
            }
        }
        eprintln!(
            "Downloaded {} packages successfully in {}",
            succeeded_packages.len().to_string().green(),
            format!("{:.0?}", start_time.elapsed()).cyan().italic()
        );
        if !failed_packages.is_empty() {
            eprintln!(
                "Failed to download {} packages: {}",
                failed_packages.len(),
                failed_packages
                    .iter()
                    .map(|p| p.to_string().red().bold().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        Ok(())
    }
}

fn get_packages_rds_url() -> String {
    format!("{}/bin/emscripten/contrib/{}/PACKAGES.rds", REPO, R_VERSION)
}

pub async fn download_packages_rds(outdir: impl AsRef<Path>) -> BundlerResult<()> {
    let client = reqwest::Client::new();
    let res = client
        .get(get_packages_rds_url())
        .send()
        .await?
        .bytes()
        .await?;
    let outfile = outdir
        .as_ref()
        .join("repo")
        .join("bin")
        .join("emscripten")
        .join("contrib")
        .join(R_VERSION)
        .join("PACKAGES.rds");
    std::fs::write(outfile, res)?;
    Ok(())
}
