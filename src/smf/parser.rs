use crate::transform_3d::{face::Face, model::Model, vertex::Vertex};
use pest::Parser;
use std::{fs, vec};

#[derive(Parser)]
#[grammar = "smf/smf.pest"]
pub struct SMFParser;

pub fn parse_file(file: &str) -> Model {
    let u_file = fs::read_to_string(file).expect("Cannot read file");

    let file = SMFParser::parse(Rule::file, &u_file)
        .expect("Failed to parse")
        .next()
        .unwrap();

    let mut vertices: Vec<Vertex> = vec![];
    let mut faces: Vec<Face> = vec![];

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::f => {
                let mut inner_rules = line.into_inner();
                let x: i32 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let y: i32 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let z: i32 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                faces.push(Face::new(x, y, z));
            }
            Rule::v => {
                let mut inner_rules = line.into_inner();
                let x: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                let y: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                let z: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                vertices.push(Vertex::new(x, y, z));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let model = Model::new(vertices, faces);
    return model;
}
