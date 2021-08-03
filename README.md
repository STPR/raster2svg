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
 -s, --size                Add the width, height and viewBox attributes
 -d, --ids                 Add the RGB colors and opacity values as id attributes
 -k, --inkscape            Add some Inkscape attributes (transparent background, pixel units, grid, snapping, ...)
 -r, --rendering           Set the shape-rendering attribute to crispEdges
 -c, --closepaths          Close the paths with the SVG Path Z command

 -z, --zoom <PERCENT>      Multiply the width and height attributes by a given percent (e.g. 250)
 -l, --color <COLOR>       Trace only one given color in hex notation RRGGBBAA (e.g. 112233FF)
 -a, --avcolor <AVCOLOR>   Avoid a given color in hex notation RRGGBBAA (e.g. 112233FF)
 -b, --bgcolor <BGCOLOR>   Add a background rectangle with a given color in hex notation RRGGBBAA (e.g. 112233FF)
 -i, --input <INPUT>       Input raster image filename (e.g. input.png)
 -o, --output <OUTPUT>     Output SVG filename (e.g. output.svg)
```
## An example with some transparent colors

```
C:\>raster2svg.exe -s -d -k -r -i rust_input.png -o rust_output.svg
```
- rust_input.png: [![rust_input.png][rust-input-img]][rust-input-url]

- rust_output.svg: [![rust_output.svg][rust-output-img]][rust-output-url]

(click on it to view the source)

## An example of a zoom by 800%

[(Thanks Nayuki for your QR Code generator)](https://www.nayuki.io/page/qr-code-generator-library)

```
C:\>raster2svg.exe -s -z 800 -k -r -l 000000ff -b ffffffff -i qrcode_input.png -o qrcode_output.svg
```
- qrcode_input.png: [![qrcode_input.png][qrcode-input-img]][qrcode-input-url]

- qrcode_output.svg: [![qrcode_output.svg][qrcode-output-img]][qrcode-output-url]

(click on it to view the source)

## Informations

- **Be careful with the size and complexity of your input image because the SVG file size grows rapidly...**
- An already compiled executable is available for Windows on GitHub: [https://github.com/STPR/raster2svg/releases](https://github.com/STPR/raster2svg/releases)

## License

raster2svg
https://github.com/STPR/raster2svg

Copyright (c) 2021, STPR - https://github.com/STPR

SPDX-License-Identifier: EUPL-1.2

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.txt](LICENSE.txt).

[cratesio-img]: https://img.shields.io/crates/v/raster2svg.svg
[cratesio-url]: https://crates.io/crates/raster2svg
[package-img]: https://img.shields.io/crates/v/contour_tracing.svg
[package-url]: https://crates.io/crates/contour_tracing
[qrcode-input-img]: https://github.com/STPR/raster2svg/raw/main/example/qrcode_input.png
[qrcode-input-url]: https://github.com/STPR/raster2svg/raw/main/example/qrcode_input.png
[qrcode-output-img]: https://github.com/STPR/raster2svg/raw/main/example/qrcode_output.svg?sanitize=true
[qrcode-output-url]: https://github.com/STPR/raster2svg/raw/main/example/qrcode_output_source.txt
[rust-input-img]: https://github.com/STPR/raster2svg/raw/main/example/rust_input.png
[rust-input-url]: https://github.com/STPR/raster2svg/raw/main/example/rust_input.png
[rust-output-img]: https://github.com/STPR/raster2svg/raw/main/example/rust_output.svg?sanitize=true
[rust-output-url]: https://github.com/STPR/raster2svg/raw/main/example/rust_output_source.txt
