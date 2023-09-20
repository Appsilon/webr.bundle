use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "Package")]
    package: Arc<str>,
    #[serde(rename = "Requirements")]
    requirements: Option<Vec<String>>,
    #[serde(rename = "Version")]
    version: Arc<str>,
    #[serde(rename = "Hash")]
    hash: Arc<str>,
}

impl Package {
    pub fn get_package(&self) -> (&str, &str) {
        (&self.package, &self.version)
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
    pub fn packages(&self) -> std::collections::btree_map::Values<String, Package> {
        self.packages.values()
    }
}
