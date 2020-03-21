# raster2svg [![crates.io][cratesio-img]][cratesio-url]

A tool that use the contour_tracing library [![Package][package-img]][package-url]

## Features

- Support **RGB colors + alpha channel** (fully transparent colors are not traced)
- Sort the colors from the most used to the less used
- Input format: a raster image file
- Output format: an SVG file

Features from the contour_tracing library: [![Package][package-img]][package-url]
- Trace contours using the Theo Pavlidis' algorithm (connectivity: 4-connected)
- Trace **outlines** in **clockwise direction**
- Trace **holes** in **counterclockwise direction**

## Options
```
 -s, --size          Add the size of the raster image
 -d, --ids           Add the RGB colors and opacity values as id attributes
 -k, --inkscape      Add some Inkscape attributes (transparent background, pixel units, ...)
 -r, --rendering     Set the shape-rendering attribute at crispEdges
 -c, --closepaths    Close the paths with the SVG Path Z command
 -h, --help          Prints help information
 -V, --version       Prints version information

 -i, --input <INPUT>      Input raster image filename (e.g. input.png)
 -o, --output <OUTPUT>    Output SVG filename (e.g. output.svg)
```
## Example

rust_input.png: [![rust_input.png][rust-input-img]][rust-input-url]

`C:\>raster2svg.exe -s -d -k -r -i rust_input.png -o rust_output.svg`

rust_output.svg: [![rust_output.svg][rust-output-img]][rust-output-url] (click on it to view the source)

## Informations

- **Be careful with the size and complexity of your input image because the SVG file size grow rapidly...**
- An already compiled executable is available for Windows on GitHub: [https://github.com/STPR/raster2svg/releases](https://github.com/STPR/raster2svg/releases)

## License

raster2svg
https://github.com/STPR/raster2svg

Copyright (c) 2020, STPR - https://github.com/STPR

SPDX-License-Identifier: EUPL-1.2

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.txt](LICENSE.txt).

[cratesio-img]: https://img.shields.io/crates/v/raster2svg.svg
[cratesio-url]: https://crates.io/crates/raster2svg
[package-img]: https://img.shields.io/crates/v/contour_tracing.svg
[package-url]: https://crates.io/crates/contour_tracing
[rust-input-img]: https://github.com/STPR/raster2svg/raw/master/example/rust_input.png
[rust-input-url]: https://github.com/STPR/raster2svg/raw/master/example/rust_input.png
[rust-output-img]: https://github.com/STPR/raster2svg/raw/master/example/rust_output.svg?sanitize=true
[rust-output-url]: https://github.com/STPR/raster2svg/raw/master/example/rust_output.svg
