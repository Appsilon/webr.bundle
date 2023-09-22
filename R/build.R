#' Bundle a Shiny App with WebR Bundle
#'
#' @param appdir The directory of the Shiny App
#    (default: current working directory)
#' @param outdir The directory to output the bundled app (default: dist)
#' @param parallel The number of packages to bundle in parallel (default: 4)
#' @export
build <- function(appdir = getwd(), outdir = file.path(getwd(), "dist"),
                  parallel = 4) {
  system2(
    "webr-bundle",
    args = c("-a", appdir, "-o", outdir, "-p", parallel, "build")
  )
}
