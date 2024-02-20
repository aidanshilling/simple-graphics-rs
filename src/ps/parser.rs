use pest::Parser;

#[derive(Parser)]
#[grammar = "ps/ps.pest"]
pub struct PSParser;

use std::{fs, vec};

pub fn parse_file(file: &str) -> Vec<Vec<Vec<f64>>> {
    let u_file = fs::read_to_string(file).expect("Cannot read file");

    let file = PSParser::parse(Rule::file, &u_file)
        .expect("Failed to parse")
        .next()
        .unwrap();

    let mut lines: Vec<Vec<f64>> = vec![];
    let mut polygons: Vec<Vec<Vec<f64>>> = vec![];
    let mut currentShape: usize = 0;

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::line => {
                let mut inner_rules = line.into_inner();
                let x1: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                let y1: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                let x2: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                let y2: f64 = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                lines.push(vec![x1, y1, x2, y2]);
            }
            Rule::lineto => {
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
                let shape = polygons.get_mut(currentShape).unwrap();
                shape.push(vec![x, y]);
            }
            Rule::moveto => {
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
                polygons.push(vec![vec![x, y]]);
            }
            Rule::stroke => {
                currentShape += 1;
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    return polygons;
}
