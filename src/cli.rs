use clap::Parser;
use std::path::{Path, PathBuf};

/// Bundle Shiny Applications for WebR in seconds!
#[derive(Parser, Debug)]
#[command(version, author)]
pub struct Args {
    /// Directory of the Shiny Application
    #[arg(short, long, default_value = ".")]
    appdir: PathBuf,

    /// Directory to output the bundle
    #[arg(short, long, default_value = "dist")]
    outdir: PathBuf,

    /// Number of packages to download in parallel
    #[arg(short, long, default_value = "4")]
    parallel: usize,

    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Bundle the Shiny Application on the specified directory
    Build,
    /// Bundle and serve the Shiny Application on the specified directory
    Serve(ServeArgs),
}

#[derive(Parser, Debug)]
pub struct ServeArgs {
    #[arg(long, default_value = "8080")]
    port: u16,
}

impl ServeArgs {
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Args {
    pub fn init() -> Self {
        Self::parse()
    }
    pub fn appdir(&self) -> &Path {
        self.appdir.as_path()
    }
    pub fn outdir(&self) -> &Path {
        self.outdir.as_path()
    }
    pub fn parallel(&self) -> usize {
        self.parallel
    }
    pub fn command(&self) -> &Command {
        &self.command
    }
}
