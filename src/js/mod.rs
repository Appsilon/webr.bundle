use std::path::Path;

use crate::renv::{Package, RenvLock};

const SHINY_JS_FILE: &str = include_str!("shiny.js");
const HTTPUV_SERVICEWORKER_JS_FILE: &str = include_str!("httpuv-serviceworker.js");

fn build_full_install_command(packages: &[Package]) -> String {
    let packages = packages
        .iter()
        .map(|pkg| pkg.get_package().0)
        .map(|pkg| format!(r#""{}""#, pkg))
        .collect::<Vec<String>>()
        .join(", ");
    [
        "export async function installPackages(webR) {",
        &format!(r#"await webR.evalRVoid(`webr::install(c({}), repos="${{window.location.href}}/repo/")`);"#, packages),
        "}",
    ]
    .join("\n")
}

fn write_shiny_js_file(outdir: impl AsRef<Path>) {
    let outfile = outdir.as_ref().join("shiny.js");
    std::fs::write(outfile, SHINY_JS_FILE).unwrap();
}

fn write_httpuv_serviceworker_js_file(outdir: impl AsRef<Path>) {
    let outfile = outdir.as_ref().join("httpuv-serviceworker.js");
    std::fs::write(outfile, HTTPUV_SERVICEWORKER_JS_FILE).unwrap();
}

fn write_install_packages(outdir: impl AsRef<Path>, renv_lock: &RenvLock) {
    let packages = renv_lock.packages().cloned().collect::<Vec<Package>>();
    let command = build_full_install_command(&packages);
    let outfile = outdir.as_ref().join("install_packages.js");
    std::fs::write(outfile, command).unwrap();
}

pub fn write_javascript(outdir: impl AsRef<Path>, renv_lock: &RenvLock) {
    write_shiny_js_file(outdir.as_ref());
    write_httpuv_serviceworker_js_file(outdir.as_ref());
    write_install_packages(outdir.as_ref(), renv_lock);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_full_install_command() {
        let packages = vec![
            Package::new("test", "0.0.1", ""),
            Package::new("test2", "0.0.1", ""),
        ];
        let command = build_full_install_command(&packages);
        assert_eq!(
            command,
            r#"export default async function installPackages(webR) {
await webR.evalRVoid(`webr::install(c("test", "test2"), repos="${window.location.href}/repo/")`);
}"#
        );
    }
}
