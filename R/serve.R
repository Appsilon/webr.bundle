#' Bundle and Serve a Shiny App with WebR Bundle
#'
#' @param appdir The directory of the Shiny App
#    (default: current working directory)
#' @param outdir The directory to output the bundled app (default: dist)
#' @param parallel The number of packages to bundle in parallel (default: 4)
#' @param port The port to serve the app on (default: 8080)
#' @export
serve <- function(appdir = getwd(), outdir = file.path(getwd(), "dist"),
                  parallel = 4, port = 8080) {
  system2(
    "webr-bundle",
    args = c(
      "-a", appdir,
      "-o", outdir,
      "-p", parallel,
      "serve",
      "--port", port
    )
  )
}
