use colored::Colorize;
use flate2::write::GzEncoder;
use flate2::Compression;
use ignore::{Walk, WalkBuilder};
use std::fs::metadata;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn build_walker(appdir: impl AsRef<Path>) -> Walk {
    let mut walk_builder = WalkBuilder::new(appdir);
    walk_builder.add_custom_ignore_filename(".webrignore");
    walk_builder.git_ignore(true);
    walk_builder.require_git(false);
    walk_builder.hidden(true);
    walk_builder.build()
}

pub fn print_note() {
    let webrignore = ".webrignore".green().bold();
    let gitignore = ".gitignore".green().bold();
    eprintln!("{:-^40}", "NOTE".yellow().bold());
    eprintln!("{webrignore} and {gitignore} are used to ignore files and directories.");
    eprintln!("Even if you don't use git, rules in {gitignore} files will be enforced.",);
    eprintln!("{:-^40}", "----".yellow().bold());
}

fn add_dist_ignore(outdir: impl AsRef<Path>) {
    let mut dist_ignore = File::create(outdir.as_ref().join(".webrignore")).unwrap();
    dist_ignore.write_all(r#"**/**"#.as_bytes()).unwrap();
}

pub fn create_dist_dir(outdir: impl AsRef<Path>) {
    std::fs::create_dir_all(outdir.as_ref()).unwrap();
    add_dist_ignore(outdir.as_ref());
}

pub fn build_bundle(appdir: impl AsRef<Path>, outdir: impl AsRef<Path>) {
    eprintln!("Building bundle...");
    print_note();
    let tar_gz = File::create(outdir.as_ref().join("app.tgz")).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    for result in build_walker(appdir) {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                let metadata = metadata(entry.path()).unwrap();
                if metadata.is_file() {
                    eprintln!(
                        "Adding {} to bundle...",
                        entry.path().display().to_string().green().bold()
                    );
                    tar.append_path(entry.path()).unwrap();
                }
            }
            Err(err) => eprintln!("{}: {}", "ERROR".red().bold(), err),
        }
    }
    tar.into_inner().unwrap().finish().unwrap();
}
