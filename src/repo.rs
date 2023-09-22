use crate::renv::{Package, RenvLock};
use colored::Colorize;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

use flate2::bufread::GzDecoder;

const R_VERSION: &str = "4.3";
const REPO: &str = "https://repo.r-wasm.org";

fn get_packages_available_url() -> String {
    format!("{}/bin/emscripten/contrib/{}/PACKAGES.gz", REPO, R_VERSION)
}

pub async fn available_packages(client: reqwest::Client) -> BTreeMap<String, Package> {
    eprintln!("Downloading available packages...");
    let res = client
        .get(get_packages_available_url())
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let mut decoder = GzDecoder::new(res.as_ref());
    let mut buffer = String::new();
    decoder.read_to_string(&mut buffer).unwrap();
    parse_available_packages(&buffer)
}

fn parse_depends(raw: &str) -> BTreeSet<String> {
    raw.split(',')
        .map(|s| s.trim())
        .filter_map(|s| s.split_whitespace().next())
        .filter(|s| !s.is_empty())
        .filter(|d| d.starts_with("R.") || d.eq(&"R"))
        .map(|dependency| dependency.to_string())
        .collect()
}

fn parse_available_packages(raw: &str) -> BTreeMap<String, Package> {
    let mut packages = BTreeMap::new();
    for block in raw.split("\n\n") {
        let mut lines = block.lines();
        let package = lines.next().unwrap().split_whitespace().last().unwrap();
        let version = lines.next().unwrap().split_whitespace().last().unwrap();
        let mut package = Package::new(package, version, "");
        // Get the dependencies
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                match key {
                    "Depends" | "Imports" => parse_depends(value)
                        .into_iter()
                        .for_each(|dependency| package.add_requirement(&dependency)),
                    _ => (),
                }
            }
        }
        packages.insert(package.get_package().0.to_string(), package);
    }
    packages
}

pub struct VesionMatcher {
    available_packages: BTreeMap<String, Package>,
}

impl VesionMatcher {
    pub async fn new(client: reqwest::Client) -> Self {
        let available_packages = available_packages(client).await;
        Self { available_packages }
    }
    // Update Renv
    pub fn sync_renv(&self, renv_lock: &mut RenvLock) {
        self.replace_libraries(renv_lock);
        self.insert_depends(renv_lock);
    }
    fn replace_libraries(&self, renv_lock: &mut RenvLock) {
        let renv_packages = renv_lock.packages_mut();
        let renv_packages_copy = renv_packages.clone();
        for (key, package) in renv_packages_copy.into_iter() {
            let available = self.available_packages.get(&key);
            match available {
                Some(available) => {
                    renv_packages.insert(key, available.clone());
                }
                None => {
                    eprintln!(
                        "Package {} not available removing from download list",
                        package.to_string().yellow().italic()
                    );
                    renv_packages.remove(&key);
                }
            };
        }
    }
    fn insert_pkg_deps(&self, pkg: &Package, renv_lock: &mut RenvLock) {
        let requirements = pkg.get_requirements().collect::<Vec<_>>();
        if requirements.is_empty() {
            return;
        }
        requirements.iter().for_each(|dependency| {
            if let Some(available) = self.available_packages.get(dependency.as_str()) {
                self.insert_pkg_deps(available, renv_lock);
                renv_lock
                    .packages_mut()
                    .insert(dependency.to_string(), available.clone());
            } else {
                renv_lock.packages_mut().remove(dependency.as_str());
            }
        })
    }
    /// In cases where version changes lead to a different set of dependencies
    fn insert_depends(&self, renv_lock: &mut RenvLock) {
        let renv_packages = renv_lock.packages_mut();
        let renv_packages_copy = renv_packages.clone();
        renv_packages_copy.into_iter().for_each(|(_, package)| {
            self.insert_pkg_deps(&package, renv_lock);
        })
    }
}
