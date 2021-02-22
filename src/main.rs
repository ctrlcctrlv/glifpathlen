extern crate clap;
extern crate glifparser;
extern crate kurbo;

use clap::{App, Arg};
use glifparser::PointType;
use kurbo::ParamCurveArclen;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct NoData;

const ACCURACY: f64 = 0.01;

fn main() {
    let matches = App::new("Path Lengths").version("0.0").author("Fredrick Brennan <copypasteⒶkittens.ph>")
        .arg(Arg::with_name("glif")
             .takes_value(true)
             .required(true)
         )
        .get_matches();

    let glif_filename = matches.value_of("glif").expect("Clap failed to require glif");
    let mut glif_file = File::open(glif_filename).expect(&format!("Unable to open file {}", glif_filename));
    let mut glif_xml = String::new();
    glif_file.read_to_string(&mut glif_xml).expect(&format!("Unable to read file {}", glif_filename));
    let glif: glifparser::Glif<NoData> = glifparser::read_ufo_glif(&glif_xml);
    let outline = glif.outline.expect("Glif contains no outline data");

    for contour in outline.iter() {
        let mut path = kurbo::BezPath::new();
        for point in contour.iter() {
            let (px, py) = (point.x as f64, point.y as f64);
            match point.ptype {
                PointType::Move => {
                    path.move_to((px, py));
                },
                PointType::Line => {
                    path.line_to((px, py));
                },
                PointType::Curve => {
                    let (ax, ay) = point.handle_or_colocated(glifparser::WhichHandle::A,|f|f,|f|f);
                    let (bx, by) = point.handle_or_colocated(glifparser::WhichHandle::B,|f|f,|f|f);
                    path.curve_to((px, py), (ax as f64, ay as f64), (bx as f64, by as f64));
                },
                PointType::QCurve => {
                    let (ax, ay) = point.handle_or_colocated(glifparser::WhichHandle::A,|f|f,|f|f);
                    path.quad_to((px, py), (ax as f64, ay as f64));
                },
                _ => todo!()
            }
        }
        //println!("{:?}", &path);
        let pathlen: f64 = path.segments().map(|seg|seg.arclen(ACCURACY)).sum();
        println!("{}", pathlen);
    }

}
