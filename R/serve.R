#' Serve a Shiny App with WebR Bundle
#'
#' @param outdir The directory where the bundle is located (default: dist)
#' @param port The port to serve the app on (default: 8080)
#' @export
serve <- function(outdir = file.path(getwd(), "dist"), port = 8080) {
  system2(
    "webr-bundle",
    args = c(
      "serve",
      "-o", outdir,
      "--port", port
    )
  )
}

#' Build and Serve a Shiny App with WebR Bundle
#' @param appdir The directory of the Shiny App
#    (default: current working directory)
#' @param outdir The directory to output the bundled app (default: dist)
#' @param parallel The number of packages to bundle in parallel (default: 4)
#' @param port The port to serve the app on (default: 8080)
#' @export
build_and_serve <- function(appdir = getwd(),
                            outdir = file.path(getwd(), "dist"),
                            parallel = 4, port = 8080) {
  build(appdir, outdir, parallel)
  serve(outdir, port)
}
