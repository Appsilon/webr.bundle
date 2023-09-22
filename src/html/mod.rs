use std::{fs::File, path::Path};

const INDEX_HTML_FILE: &str = include_str!("index.html");

pub fn write_index_html_file(outdir: impl AsRef<Path>) {
    let outfile = outdir.as_ref().join("index.html");
    std::fs::write(outfile, INDEX_HTML_FILE).unwrap();
}
