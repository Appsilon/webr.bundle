use clap::Parser;
use std::path::{Path, PathBuf};

/// Bundle Shiny Applications for WebR in seconds!
#[derive(Parser, Debug)]
#[command(version, author)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Bundle the Shiny Application on the specified directory
    Build(BuildArgs),
    /// Bundle and serve the Shiny Application on the specified directory
    Serve(ServeArgs),
}

#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// Directory of the Shiny Application
    #[arg(short, long, default_value = ".")]
    appdir: PathBuf,

    /// Directory to output the bundle
    #[arg(short, long, default_value = "dist")]
    outdir: PathBuf,

    /// Number of packages to download in parallel
    #[arg(short, long, default_value = "4")]
    parallel: usize,
}

#[derive(Parser, Debug)]
pub struct ServeArgs {
    /// Directory to serve (where the bundle is located)
    #[arg(short, long, default_value = "dist")]
    outdir: PathBuf,

    /// Port to bind the server to
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

impl BuildArgs {
    pub fn appdir(&self) -> &Path {
        self.appdir.as_path()
    }
    pub fn outdir(&self) -> &Path {
        self.outdir.as_path()
    }
    pub fn parallel(&self) -> usize {
        self.parallel
    }
}

impl ServeArgs {
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn outdir(&self) -> &Path {
        self.outdir.as_path()
    }
}

impl Args {
    pub fn init() -> Self {
        Self::parse()
    }
    pub fn command(&self) -> &Command {
        &self.command
    }
}
