/*
 * raster2svg 1.0.9
 * https://github.com/STPR/raster2svg
 *
 * Copyright (c) 2020, STPR - https://github.com/STPR
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

extern crate clap;
extern crate image;
extern crate contour_tracing;

use image::{GenericImageView, Pixel};
use contour_tracing::bits_to_paths;
use std::{process, collections::BTreeMap, fs::File, io::prelude::*, iter::FromIterator};

const OPACITY: [&'static str; 255] = ["0", "0.00392157", "0.00784314", "0.01176471", "0.01568627", "0.01960784", "0.02352941", "0.02745098", "0.03137255", "0.03529412", "0.03921569", "0.04313725", "0.04705882", "0.05098039", "0.05490196", "0.05882353", "0.0627451", "0.06666667", "0.07058824", "0.0745098", "0.07843137", "0.08235294", "0.08627451", "0.09019608", "0.09411765", "0.09803922", "0.10196078", "0.10588235", "0.10980392", "0.11372549", "0.11764706", "0.12156863", "0.1254902", "0.12941176", "0.13333333", "0.1372549", "0.14117647", "0.14509804", "0.14901961", "0.15294118", "0.15686275", "0.16078431", "0.16470588", "0.16862745", "0.17254902", "0.17647059", "0.18039216", "0.18431373", "0.18823529", "0.19215686", "0.19607843", "0.2", "0.20392157", "0.20784314", "0.21176471", "0.21568627", "0.21960784", "0.22352941", "0.22745098", "0.23137255", "0.23529412", "0.23921569", "0.24313725", "0.24705882", "0.25098039", "0.25490196", "0.25882353", "0.2627451", "0.26666667", "0.27058824", "0.2745098", "0.27843137", "0.28235294", "0.28627451", "0.29019608", "0.29411765", "0.29803922", "0.30196078", "0.30588235", "0.30980392", "0.31372549", "0.31764706", "0.32156863", "0.3254902", "0.32941176", "0.33333333", "0.3372549", "0.34117647", "0.34509804", "0.34901961", "0.35294118", "0.35686275", "0.36078431", "0.36470588", "0.36862745", "0.37254902", "0.37647059", "0.38039216", "0.38431373", "0.38823529", "0.39215686", "0.39607843", "0.4", "0.40392157", "0.40784314", "0.41176471", "0.41568627", "0.41960784", "0.42352941", "0.42745098", "0.43137255", "0.43529412", "0.43921569", "0.44313725", "0.44705882", "0.45098039", "0.45490196", "0.45882353", "0.4627451", "0.46666667", "0.47058824", "0.4745098", "0.47843137", "0.48235294", "0.48627451", "0.49019608", "0.49411765", "0.49803922", "0.50196078", "0.50588235", "0.50980392", "0.51372549", "0.51764706", "0.52156863", "0.5254902", "0.52941176", "0.53333333", "0.5372549", "0.54117647", "0.54509804", "0.54901961", "0.55294118", "0.55686275", "0.56078431", "0.56470588", "0.56862745", "0.57254902", "0.57647059", "0.58039216", "0.58431373", "0.58823529", "0.59215686", "0.59607843", "0.6", "0.60392157", "0.60784314", "0.61176471", "0.61568627", "0.61960784", "0.62352941", "0.62745098", "0.63137255", "0.63529412", "0.63921569", "0.64313725", "0.64705882", "0.65098039", "0.65490196", "0.65882353", "0.6627451", "0.66666667", "0.67058824", "0.6745098", "0.67843137", "0.68235294", "0.68627451", "0.69019608", "0.69411765", "0.69803922", "0.70196078", "0.70588235", "0.70980392", "0.71372549", "0.71764706", "0.72156863", "0.7254902", "0.72941176", "0.73333333", "0.7372549", "0.74117647", "0.74509804", "0.74901961", "0.75294118", "0.75686275", "0.76078431", "0.76470588", "0.76862745", "0.77254902", "0.77647059", "0.78039216", "0.78431373", "0.78823529", "0.79215686", "0.79607843", "0.8", "0.80392157", "0.80784314", "0.81176471", "0.81568627", "0.81960784", "0.82352941", "0.82745098", "0.83137255", "0.83529412", "0.83921569", "0.84313725", "0.84705882", "0.85098039", "0.85490196", "0.85882353", "0.8627451", "0.86666667", "0.87058824", "0.8745098", "0.87843137", "0.88235294", "0.88627451", "0.89019608", "0.89411765", "0.89803922", "0.90196078", "0.90588235", "0.90980392", "0.91372549", "0.91764706", "0.92156863", "0.9254902", "0.92941176", "0.93333333", "0.9372549", "0.94117647", "0.94509804", "0.94901961", "0.95294118", "0.95686275", "0.96078431", "0.96470588", "0.96862745", "0.97254902", "0.97647059", "0.98039216", "0.98431373", "0.98823529", "0.99215686", "0.99607843"];

fn string_to_color(string: &str) -> Option<image::Rgba<u8>> {
    for c in string.chars() {
        if !c.is_digit(16) {
            return None;
        }
    }
    unsafe {
        if string.chars().count() == 8 {
            return Some(Pixel::from_channels(u8::from_str_radix(string.get_unchecked(0..2), 16).unwrap(), u8::from_str_radix(string.get_unchecked(2..4), 16).unwrap(), u8::from_str_radix(string.get_unchecked(4..6), 16).unwrap(), u8::from_str_radix(string.get_unchecked(6..8), 16).unwrap()))
        } else {
            return None
        };
    }
}

fn main() -> Result<(), std::io::Error> {

    use clap::{load_yaml, App, AppSettings};
    let cli_yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(cli_yaml).setting(AppSettings::ArgRequiredElseHelp).get_matches();

    let size: bool = m.is_present("size");
    let zoom: u32;
    if m.is_present("PERCENT") {
        zoom = match m.value_of("PERCENT") {
            Some(zoom) => zoom.parse().unwrap_or(100),
            None => {
                println!("{}", m.usage());
                process::exit(1);
            }
        };
    } else {
        zoom = 100;
    }
    let color_rgba: image::Rgba<u8>;
    if m.is_present("COLOR") {
        color_rgba = match string_to_color(m.value_of("COLOR").unwrap()) {
            Some(color_rgba) => color_rgba,
            None => {
                println!("{}", m.usage());
                process::exit(1);
            }
        };
    } else {
        color_rgba = Pixel::from_channels(0, 0, 0, 0);
    }
    let ids: bool = m.is_present("ids");
    let inkscape: bool = m.is_present("inkscape");
    let rendering: bool = m.is_present("rendering");
    let closepaths: bool = m.is_present("closepaths");
    let input = match m.value_of("INPUT") {
        Some(input) => input,
        None => {
            println!("{}", m.usage());
            process::exit(1);
        }
    };
    let output = match m.value_of("OUTPUT") {
        Some(output) => output,
        None => {
            println!("{}", m.usage());
            process::exit(1);
        }
    };

    let raster = image::open(input).expect("Error while opening the raster image file !");
    let (raster_x, raster_y) = raster.dimensions();
    let zoomed_x: u32;
    let zoomed_y: u32;
    if m.is_present("PERCENT") {
        zoomed_x = (raster_x as f64 * (zoom as f64 / 100.0)).round() as u32;
        zoomed_y = (raster_y as f64 * (zoom as f64 / 100.0)).round() as u32;
    } else {
        zoomed_x = raster_x;
        zoomed_y = raster_y;
    }

    // FIRST PHASE: Count and sort the colors from the most used to the less used
    let mut number_of_colors: usize = 0;
    let mut sorted_colors;
    if m.is_present("COLOR") {
        for p in raster.pixels() {
            if p.2 == color_rgba {
                number_of_colors = 1;
                break;
            };
        }
        sorted_colors = vec![([color_rgba[0], color_rgba[1], color_rgba[2], color_rgba[3]], 1)];
    } else {
        let mut colors = BTreeMap::new(); // Use BTreeMap instead of HashMap to ensure the same arrangement each time (but slower)
        for p in raster.pixels() {
            if p.2[3] != 0 {
                *colors.entry((p.2).0).or_insert(0) += 1;
            }
        }
        number_of_colors = colors.len();
        sorted_colors = Vec::from_iter(colors);
        sorted_colors.sort_by(|&(_, a), &(_, b)| a.cmp(&b).reverse());
    }
    // END OF THE FIRST PHASE

    let create_file: bool = if number_of_colors > 0 { true } else { false };
    let file_ref = if create_file { Some(File::create(output)?) } else { None };
    if create_file {
        if let Some(mut fsvg) = file_ref.as_ref() {
            write!(fsvg, "<svg xmlns=\"http://www.w3.org/2000/svg\"")?;
            if size { write!(fsvg, " width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\"", zoomed_x, zoomed_y, raster_x, raster_y)?; }
            if rendering { write!(fsvg, " shape-rendering=\"crispEdges\"")?; }
            if inkscape { write!(fsvg, " xmlns:sodipodi=\"http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd\" xmlns:inkscape=\"http://www.inkscape.org/namespaces/inkscape\"")?; }
            write!(fsvg, ">\n")?;
            if inkscape { writeln!(fsvg, "<sodipodi:namedview units=\"px\" inkscape:document-units=\"px\" inkscape:pagecheckerboard=\"true\" showborder=\"true\" borderlayer=\"true\" showgrid=\"true\" inkscape:showpageshadow=\"false\" inkscape:snap-global=\"true\" inkscape:snap-bbox=\"true\" inkscape:bbox-nodes=\"true\" inkscape:snap-nodes=\"false\" inkscape:snap-others=\"false\" inkscape:snap-grids=\"true\" inkscape:snap-to-guides=\"false\">\n\
                                          <inkscape:grid type=\"xygrid\" id=\"pixelgrid\" visible=\"false\" originx=\"0\" originy=\"0\" spacingx=\"1\" spacingy=\"1\" empspacing=\"1\" snapvisiblegridlinesonly=\"false\"/>\
                                          </sodipodi:namedview>")?; }

            for color in sorted_colors.into_iter() {
                let red: u8 = color.0[0];
                let green: u8 = color.0[1];
                let blue: u8 = color.0[2];
                let alpha: usize = color.0[3] as usize;

                if alpha == 255 {
                    write!(fsvg, "<path ")?;
                    if ids {
                        write!(fsvg, "id=\"{}_{}_{}\" ", red, green, blue)?;
                    }
                    write!(fsvg, "fill=\"rgb({},{},{})\" d=\"", red, green, blue)?;
                } else {
                    write!(fsvg, "<path ")?;
                    if ids {
                        write!(fsvg, "id=\"{}_{}_{}_{}\" ", red, green, blue, OPACITY[alpha])?;
                    }
                    write!(fsvg, "fill=\"rgb({},{},{})\" opacity=\"{}\" d=\"", red, green, blue, OPACITY[alpha])?;
                }

                // SECOND PHASE: Fill an array of bits for one color
                let mut bits = vec![vec![0i8; raster_x as usize]; raster_y as usize];
                for p in raster.pixels() {
                    if p.2 == image::Rgba(color.0) {
                        bits[p.1 as usize][p.0 as usize] = 1;
                    }
                }
                // END OF THE SECOND PHASE

                // THIRD PHASE: Use the contour_tracing library
                write!(fsvg, "{}", bits_to_paths(bits, closepaths))?;
                // END OF THE THIRD PHASE

                writeln!(fsvg, "\"/>")?;
            }
            writeln!(fsvg, "</svg>")?;
        }
    }

    println!("raster2svg 1.0.9\nCopyright (c) 2020, STPR - https://github.com/STPR\nFor more information, please visit https://crates.io/crates/raster2svg\n");
    println!("Input size: {} x {}", raster_x, raster_y);
    if m.is_present("PERCENT") {
        println!("Zoom: {} %\nZoomed size: {} x {}", zoom, zoomed_x, zoomed_y);
    }
    println!("Number of colors found: {}\n", number_of_colors);
    if create_file { println!("Done.") } else { println!("No output file has been created...") };
    Ok(())
}
