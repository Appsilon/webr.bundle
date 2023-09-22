# WebR Bundle

A tool for bundling Shiny Apps with WebAssembly.

## R Package

### Installation

Install the package from GitHub:

```R
# install.packages("remotes")
pak::pak("andyquinterom/webr.bundle")
```

After installing the package, you need to install the `webr-bundle` CLI. You can do this by running the following function from R:

```R
webr.bundle::install_from_source()
```

This will install Rust (if not already installed) and then build and install the `webr-bundle` CLI.

### Usage

#### Bundle a Shiny App

In order to bundle a Shiny App, it must use [renv](https://rstudio.github.io/renv/) to manage its dependencies. This is because `webr.bundle` will use the `renv.lock` file to discover the dependencies of the app and try to bundle them.

By default, `webr.bundle` will bundle the app in the current working directory and place the bundled app in the `dist` directory. You can specify a different app directory by
specifying the `appdir` argument (first argument) and the output directory by specifying the `outdir` argument (second argument).

```R
# Bundle the app in the current working directory
webr.bundle::build()
```

```R
# Bundle the app in the my-shiny-app directory and place the bundled app in the my-bundled-app directory
webr.bundle::build("my-shiny-app", "my-bundled-app")
```

#### Run a bundled Shiny App

You can run the bundled shiny app with any HTTP server, however, `webr.bundle` provides a simple HTTP server that can be used to run the app. `webr.bundle::serve` uses the same API as
`webr.bundle::build` and will build the app if it hasn't been built yet. However, you can also specify a port to bind the HTTP server on.

```R
# Run the bundled app in the current working directory
# on port 8080
webr.bundle::serve()
```

```R
# Run the shiny app (using WebR) in the my-shiny-app directory
# on port 3000
webr.bundle::serve("my-shiny-app", port = 3000)
```

## CLI

### Installation

#### Build from source

First, install [Rust](https://www.rust-lang.org/tools/install) by running the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone this repository and `cd` into it:

```bash
git clone https://github.com/andyquinterom/webr.bundle.git
cd webr.bundle
```

Finally, build the project:

```bash
cargo install --path inst/bin/cli
```

Now you should be able to run the `webr-bundle` command.

```bash
webr-bundle --help
```

### Usage

#### Bundle a Shiny App

In order to bundle a Shiny App, it must use [renv](https://rstudio.github.io/renv/) to manage its dependencies. This is because `webr-bundle` will use the `renv.lock` file to discover the dependencies of the app and try to bundle them.

To bundle a Shiny App, we need to `cd` into the app's directory and run the following command:

```bash
webr-bundle build
```

This will create a new `dist` directory with the bundled app. If you want to specify a different output directory or a different app directory, you can use (`-o`, `--outdir`) and (`-a`, `--appdir`) respectively.

```bash
# ./
# ├── ./my-shiny-app (Directory containing the Shiny App)
# │   ├── app.R
# │   └── renv.lock
# └── ./my-bundled-app (Output directory)
webr-bundle -o my-bundled-app -a my-shiny-app build
```

#### Run a bundled Shiny App

You can run the bundled shiny app with any HTTP server, however, `webr-bundle` provides a simple HTTP server that can be used to run the app.

```bash
webr-bundle serve
```

This command will build the app and serve it at `http://localhost:8080`. If you want to specify a different port, you can use (`-p`, `--port`).

```bash
webr-bundle serve -p 3000
```

You can still use (`-o`, `--outdir`) and (`-a`, `--appdir`) to specify the output directory and the app directory respectively.

```bash
webr-bundle -o my-bundled-app -a my-shiny-app serve
```
