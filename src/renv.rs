use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "Package")]
    package: Arc<str>,
    #[serde(rename = "Requirements")]
    #[serde(default)]
    requirements: BTreeSet<String>,
    #[serde(rename = "Version")]
    version: Arc<str>,
    #[serde(rename = "Hash")]
    hash: Arc<str>,
}

impl Package {
    pub fn get_package(&self) -> (&str, &str) {
        (&self.package, &self.version)
    }
    pub fn new(package: &str, version: &str, hash: &str) -> Self {
        Self {
            package: package.into(),
            requirements: BTreeSet::new(),
            version: version.into(),
            hash: hash.into(),
        }
    }
    pub fn add_requirement(&mut self, requirement: &str) {
        self.requirements.insert(requirement.into());
    }
    pub fn get_requirements(&self) -> std::collections::btree_set::Iter<String> {
        self.requirements.iter()
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (package, version) = self.get_package();
        write!(f, "{} ({})", package, version)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenvLock {
    #[serde(rename = "Packages")]
    packages: BTreeMap<String, Package>,
}

impl RenvLock {
    pub fn read_from_file(appdir: impl AsRef<std::path::Path>) -> Self {
        let renv_lock = std::fs::File::open(appdir.as_ref().join("renv.lock")).unwrap();
        serde_json::from_reader::<_, RenvLock>(renv_lock).unwrap()
    }
    pub fn packages(&self) -> std::collections::btree_map::Values<String, Package> {
        self.packages.values()
    }
    pub fn packages_mut(&mut self) -> &mut BTreeMap<String, Package> {
        &mut self.packages
    }
    pub fn contains(&self, package: &str) -> bool {
        self.packages.contains_key(package)
    }
}
