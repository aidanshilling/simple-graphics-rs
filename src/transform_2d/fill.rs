use std::{collections::HashMap, vec};

use itertools::Itertools;

use super::transform::compute_intersect;

pub fn fill_polygons(polygons: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    let mut matrix = generate_pbm_matrix(501, 501);
    for polygon in polygons {
        let mut scan_map = generate_scanline_map(0, 501);
        for edge in polygon {
            let mut scan_line = 0;
            let ymin;
            let ymax;
            if edge[1] > edge[3] {
                ymin = edge[3];
                ymax = edge[1];
            } else {
                ymin = edge[1];
                ymax = edge[3];
            }
            if ymin == ymax {
                continue;
            }
            while scan_line < 501 {
                if ymin <= scan_line && scan_line < ymax {
                    scan_map.get_mut(&scan_line).unwrap().push(edge.clone());
                }
                scan_line += 1;
            }
        }
        for scan_line in scan_map.keys().sorted() {
            let mut intersections = vec![];
            for edge in &scan_map[scan_line] {
                let intersect = compute_intersect(
                    vec![edge[0] as f64, edge[1] as f64],
                    vec![edge[2] as f64, edge[3] as f64],
                    &[0 as f64, *scan_line as f64, 501 as f64, *scan_line as f64],
                );
                intersections.push(vec![intersect[0] as i32, intersect[1] as i32]);
            }
            intersections.sort();
            if intersections.len() > 0 {
                for i in (0..intersections.len()).step_by(2) {
                    let mut pixel = intersections[i][0];
                    let stop = intersections[i + 1][0];
                    while pixel < stop && pixel < 501 {
                        matrix[*scan_line as usize][pixel as usize] = 1;
                        pixel += 1;
                    }
                }
            }
        }
    }
    return matrix;
}

pub fn generate_scanline_map(k: i32, p: i32) -> HashMap<i32, Vec<Vec<i32>>> {
    let mut i = k;
    let mut out: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    while i <= p {
        out.insert(i, vec![]);
        i += 1;
    }
    return out;
}

pub fn generate_pbm_matrix(xsize: i32, ysize: i32) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = vec![];
    let mut i: usize = 0;
    while i < ysize as usize {
        let mut j: usize = 0;
        out.push(vec![]);
        while j < xsize as usize {
            out.get_mut(i).unwrap().push(0);
            j += 1;
        }
        i += 1;
    }
    return out;
}

pub fn write_pbm(matrix: Vec<Vec<i32>>) {
    let mut i = 1;
    for row in matrix.iter().rev() {
        for item in row.iter() {
            print!("{}", item);
            if i == 70 {
                print!("\n");
                i = 1;
            } else {
                i += 1;
            }
        }
    }
}
