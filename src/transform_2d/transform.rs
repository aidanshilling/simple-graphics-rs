use std::vec;

#[derive(PartialEq)]
pub enum Plane {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

pub fn clip_polygon(
    p: Vec<Vec<f64>>,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> Vec<Vec<f64>> {
    let bounds = vec![Plane::TOP, Plane::BOTTOM, Plane::LEFT, Plane::RIGHT];
    let mut p1: Vec<Vec<f64>> = p;
    let mut current_poly: Vec<Vec<f64>>;
    for edge in bounds.iter() {
        current_poly = p1.clone();
        p1.clear();

        for (i, v) in current_poly.iter().enumerate() {
            let current = v;
            let prev;
            if i == 0 {
                prev = &current_poly[&current_poly.len() - 1];
            } else {
                prev = &current_poly[i - 1];
            }

            let p_out = is_outside(prev.clone(), edge, min_x, min_y, max_x, max_y);
            let c_out = is_outside(current.to_vec(), edge, min_x, min_y, max_x, max_y);

            let intersect = get_intersect(
                current.to_vec(),
                prev.to_vec(),
                edge,
                min_x,
                min_y,
                max_x,
                max_y,
            );

            if !c_out {
                if p_out {
                    p1.push(intersect);
                }
                p1.push(current.to_vec());
            } else if !p_out {
                p1.push(intersect);
            }
        }
    }
    return p1;
}

pub fn get_intersect(
    c: Vec<f64>,
    p: Vec<f64>,
    edge: &Plane,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> Vec<f64> {
    return match edge {
        Plane::TOP => compute_intersect(p, c, &[min_x, max_y, max_x, max_y]),
        Plane::BOTTOM => compute_intersect(p, c, &[min_x, min_y, max_x, min_y]),
        Plane::LEFT => compute_intersect(p, c, &[min_x, min_y, min_x, max_y]),
        Plane::RIGHT => compute_intersect(p, c, &[max_x, min_y, max_x, max_y]),
    };
}

pub fn is_outside(
    point: Vec<f64>,
    plane: &Plane,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> bool {
    let x = point[0];
    let y = point[1];
    match plane {
        Plane::TOP => y > max_y,
        Plane::BOTTOM => y < min_y,
        Plane::LEFT => x < min_x,
        Plane::RIGHT => x > max_x,
    }
}

pub fn compute_intersect(p: Vec<f64>, c: Vec<f64>, plane: &[f64; 4]) -> Vec<f64> {
    let x1 = p[0];
    let y1 = p[1];
    let x2 = c[0];
    let y2 = c[1];
    let x3 = plane[0];
    let y3 = plane[1];
    let x4 = plane[2];
    let y4 = plane[3];

    let x = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
    let y = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

    return vec![x, y];
}

pub fn scale_polygon_to_viewport(
    p: Vec<Vec<f64>>,
    umin: f64,
    vmin: f64,
    umax: f64,
    vmax: f64,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
) -> Vec<Vec<f64>> {
    let mut v_polygon: Vec<Vec<f64>> = vec![];
    for vertex in p {
        let mut v_vertex = vertex.clone();
        // v_vertex = translate(v_vertex, umin, vmin);
        v_vertex = scale_to_viewport(
            v_vertex,
            (umax - umin) / (xmax - xmin),
            (vmax - vmin) / (ymax - ymin),
        );
        // v_vertex = transform(v_vertex, -xmin, -ymin)
        v_vertex = translate(v_vertex, (umax + umin) / 2.0, (vmax + vmin) / 2.0);
        v_polygon.push(v_vertex);
    }
    return v_polygon;
}

pub fn abs(line: Vec<f64>) -> Vec<f64> {
    let x0 = line[0].abs();
    let y0 = line[1].abs();
    return vec![x0, y0];
}

pub fn transform(line: Vec<f64>, a: f64, b: f64) -> Vec<f64> {
    let x0 = line[0] - a;
    let y0 = line[1] - b;
    return vec![x0, y0];
}

pub fn translate(line: Vec<f64>, dx: f64, dy: f64) -> Vec<f64> {
    let x0 = line[0] + dx;
    let y0 = line[1] + dy;
    return vec![x0, y0];
}

pub fn scale(line: Vec<f64>, scale: f64) -> Vec<f64> {
    let x0 = line[0] * scale;
    let y0 = line[1] * scale;
    return vec![x0, y0];
}

pub fn scale_to_viewport(vertex: Vec<f64>, xscale: f64, yscale: f64) -> Vec<f64> {
    let x0 = vertex[0] * xscale;
    let y0 = vertex[1] * yscale;
    return vec![x0, y0];
}

pub fn rotate(line: Vec<f64>, r: f64) -> Vec<f64> {
    let x0 = line[0] * r.cos() - line[1] * r.sin();
    let y0 = line[0] * r.sin() + line[1] * r.cos();
    return vec![x0, y0];
}
