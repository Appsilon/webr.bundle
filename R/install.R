get_os <- function() {
  sysinf <- Sys.info()
  if (!is.null(sysinf)) {
    os <- sysinf["sysname"]
    if (os == "Darwin")
      os <- "osx"
  } else {
    os <- .Platform$OS.type
    if (grepl("^darwin", R.version$os))
      os <- "osx"
    if (grepl("linux-gnu", R.version$os))
      os <- "linux"
  }
  tolower(os)
}

check_cargo <- function() {
  os <- get_os()
  if (os %in% c("osx", "linux")) {
    # Check if Cargo is installed
    return(system("cargo --version", intern = TRUE) == 0)
  } else {
    stop("Unsupported OS")
  }
}

install_rust <- function() {
  if (!check_cargo()) {
    # Install Rust
    system("curl https://sh.rustup.rs -sSf | sh -s -- -y")
    # Add Cargo to PATH
    Sys.setenv(PATH = paste(Sys.getenv("PATH"), "~/.cargo/bin", sep = ":"))
  }
}

cargo_install <- function(path) {
  # Install Rust dependencies
  system2("cargo", args = c("install", "--path", path))
}

download_webr_bundle_source <- function() {
  tempdir <- tempfile()
  # Clone webr-bundle
  git2r::clone(
    url = "https://github.com/andyquinterom/webr-bundle.git",
    local_path = tempdir
  )
  return(tempdir)
}

#' Install webr-bundle from source
#'
#' This function installs webr-bundle from source.
#' It is only supported on macOS and Linux and
#' requires Rust to be installed. If Rust is not
#' installed, it will be installed automatically.
#' @export
install_from_source <- function() {
  # Install Rust
  install_rust()
  # Download webr-bundle source
  tempdir <- download_webr_bundle_source()
  # Install webr-bundle
  cargo_install(tempdir)
}