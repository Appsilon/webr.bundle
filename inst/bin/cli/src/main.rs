use webr_bundle::{
    bundle::{build_bundle, create_dist_dir},
    cli::Args,
    cli::Command,
    download::download_packages_rds,
    errors::BundlerResult,
    html::write_index_html_file,
    js::write_javascript,
    logs,
    renv::RenvLock,
};

#[tokio::main]
async fn main() {
    logs::init();
    let args = Args::init();
    if let Err(err) = logic(args).await {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}

async fn logic(args: Args) -> BundlerResult<()> {
    match args.command() {
        Command::Build(build_args) => {
            let appdir = build_args.appdir();
            let outdir = build_args.outdir();
            create_dist_dir(outdir)?;
            build_bundle(appdir, outdir)?;
            let mut renv_lock = RenvLock::read_from_file(appdir)?;
            renv_lock.download(outdir, build_args.parallel()).await?;
            download_packages_rds(outdir).await?;
            write_javascript(outdir, &renv_lock)?;
            write_index_html_file(outdir)?;
        }
        Command::Serve(serve_args) => {
            let port = serve_args.port();
            let outdir = serve_args.outdir().into();
            eprintln!("Serving on http://localhost:{port}");
            webr_bundle::serve::server(outdir, port).await?
        }
    }
    Ok(())
}
