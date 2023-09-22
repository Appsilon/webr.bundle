use webr_bundle::{
    bundle::create_dist_dir, cli::Args, cli::Command, download::download_packages_rds,
    html::write_index_html_file, js::write_javascript, logs, renv::RenvLock,
};

#[tokio::main]
async fn main() {
    logs::init();
    let args = Args::init();
    let appdir = args.appdir();
    let outdir = args.outdir();
    create_dist_dir(outdir);
    webr_bundle::bundle::build_bundle(appdir, outdir);
    let mut renv_lock = RenvLock::read_from_file(appdir);
    renv_lock.download(outdir, args.parallel()).await;
    download_packages_rds(outdir).await;
    write_javascript(outdir, &renv_lock);
    write_index_html_file(outdir);
    match args.command() {
        Command::Build => eprintln!("Bundled and ready to ship!"),
        Command::Serve(serve_args) => {
            eprintln!("Serving on http://localhost:{}", serve_args.port());
            webr_bundle::serve::server(outdir.to_path_buf(), serve_args.port())
                .await
                .unwrap()
        }
    }
}
