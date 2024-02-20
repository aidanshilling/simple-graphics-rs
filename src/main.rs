#![allow(non_snake_case)]
extern crate clap;
extern crate pest;

#[macro_use]
extern crate pest_derive;

mod args;
mod ps;
mod smf;
mod transform_2d;
mod transform_3d;

use std::path::Path;

use smf::parser::parse_file;
use transform_2d::transform::scale_polygon_to_viewport;

use crate::{
    args::Args,
    transform_2d::transform::clip_polygon,
    transform_3d::projection::{ParallelPipeline, PerspectivePipeline, ProjectionData},
};
use clap::Parser;

/*
    Main method:
    Parses command line arguments, get's line data and writes to file
*/
fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    if !Path::new(&args.file).exists() {
        println!("file: '{}' not found.", args.file);
        return Ok(());
    }

    let model = parse_file(&args.file);
    let data = ProjectionData::new(args.clone());

    print!("%%BeginSetup\n");
    print!("\t<< /PageSize [{} {}] >> setpagedevice\n", 501, 501);
    print!("%%EndSetup\n\n");
    print!("%%%BEGIN\n");

    if args.P {
        let par = ParallelPipeline::new(data.clone());
        for p in par.normalize(model).iter() {
            let a = args.clone();
            let p_c = clip_polygon(p.clone(), -1.0, -1.0, 1.0, 1.0);
            let p_s = scale_polygon_to_viewport(
                p_c.clone(),
                a.j as f64,
                a.k as f64,
                a.o as f64,
                a.p as f64,
                -1.0,
                -1.0,
                1.0,
                1.0,
            );

            if p_s.len() == 0 {
                continue;
            }

            let begin = p_s.get(0).unwrap();
            print!("{} {} moveto\n", begin[0], begin[1]);
            for (i, vertex) in p_s.iter().enumerate() {
                if i > 0 {
                    print!("{} {} lineto\n", vertex[0], vertex[1]);
                }
            }

            print!("stroke\n");
        }
    } else {
        let d: f64 = args.z / (args.B - args.z);
        let per = PerspectivePipeline::new(data.clone());
        for p in per.normalize(model).iter() {
            let p_c = clip_polygon(p.clone(), -(d.abs()), -(d.abs()), d.abs(), d.abs());
            let p_s = scale_polygon_to_viewport(
                p_c.clone(),
                0.0,
                0.0,
                500.0,
                500.0,
                -(d.abs()),
                -(d.abs()),
                d.abs(),
                d.abs(),
            );

            if p_s.len() == 0 {
                continue;
            }

            let begin = p_s.get(0).unwrap();
            print!("{} {} moveto\n", begin[0], begin[1]);
            for (i, vertex) in p_s.iter().enumerate() {
                if i > 0 {
                    print!("{} {} lineto\n", vertex[0], vertex[1]);
                }
            }

            print!("stroke\n");
        }
    }

    print!("%%%");

    Ok(())
}
