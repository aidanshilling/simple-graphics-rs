use nalgebra::{RowVector3, RowVector4, SMatrix};

use crate::args::Args;

use super::model::Model;

#[derive(Debug, Clone)]
pub struct ProjectionData {
    args: Args,
    t_vrp: SMatrix<f64, 4, 4>,
    r: SMatrix<f64, 4, 4>,
    sh: SMatrix<f64, 4, 4>,
}

impl ProjectionData {
    pub fn new(args: Args) -> Self {
        let vpn: RowVector3<f64> = RowVector3::new(args.q, args.r, args.w);
        let vup: RowVector3<f64> = RowVector3::new(args.Q, args.R, args.W);

        let t_vrp: SMatrix<f64, 4, 4> = SMatrix::from_rows(&[
            RowVector4::new(1.0, 0.0, 0.0, -args.X),
            RowVector4::new(0.0, 1.0, 0.0, -args.Y),
            RowVector4::new(0.0, 0.0, 1.0, -args.Z),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        let r_z: RowVector3<f64> = vpn.normalize();
        let r_x: RowVector3<f64> = (vup.cross(&r_z)).normalize();
        let r_y: RowVector3<f64> = r_z.cross(&r_x);

        let r: SMatrix<f64, 4, 4> = SMatrix::from_row_slice(&[
            r_x[0], r_x[1], r_x[2], 0.0, r_y[0], r_y[1], r_y[2], 0.0, r_z[0], r_z[1], r_z[2], 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        let sh: SMatrix<f64, 4, 4> = SMatrix::from_rows(&[
            RowVector4::new(1.0, 0.0, (0.5 * (args.u + args.U) - args.x) / args.z, 0.0),
            RowVector4::new(0.0, 1.0, (0.5 * (args.v + args.V) - args.y) / args.z, 0.0),
            RowVector4::new(0.0, 0.0, 1.0, 0.0),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        ProjectionData { args, t_vrp, r, sh }
    }
}

#[derive(Debug, Clone)]
pub struct ParallelPipeline {
    normal: SMatrix<f64, 4, 4>,
}

impl ParallelPipeline {
    pub fn new(data: ProjectionData) -> Self {
        let args = data.args;

        let t: SMatrix<f64, 4, 4> = SMatrix::from_rows(&[
            RowVector4::new(1.0, 0.0, 0.0, -(args.U + args.u) / 2.0),
            RowVector4::new(0.0, 1.0, 0.0, -(args.V + args.v) / 2.0),
            RowVector4::new(0.0, 0.0, 1.0, -args.F),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        let s: SMatrix<f64, 4, 4> = SMatrix::from_rows(&[
            RowVector4::new(2.0 / (args.U - args.u), 0.0, 0.0, 0.0),
            RowVector4::new(0.0, 2.0 / (args.V - args.v), 0.0, 0.0),
            RowVector4::new(0.0, 0.0, 1.0 / (args.F - args.B), 0.0),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        let normal: SMatrix<f64, 4, 4> = s * (t * (data.sh * (data.r * data.t_vrp)));

        ParallelPipeline { normal }
    }

    pub fn normal(self) -> SMatrix<f64, 4, 4> {
        self.normal
    }

    pub fn normalize(self, model: Model) -> Vec<Vec<Vec<f64>>> {
        let mut v_out: Vec<Vec<Vec<f64>>> = vec![];
        for polygon in model.polygons.iter() {
            let mut vertices: Vec<Vec<f64>> = vec![];
            for vertex in polygon.vertices.iter().rev() {
                let v_h = self.normal * vertex.clone().to_vector();
                let x = v_h[0];
                let y = v_h[1];
                let v = vec![x, y];
                vertices.push(v);
            }
            vertices.push(vertices[0].clone());
            v_out.push(vertices);
        }
        v_out
    }
}

#[derive(Debug, Clone)]
pub struct PerspectivePipeline {
    normal: SMatrix<f64, 4, 4>,
    d: f64,
}

impl PerspectivePipeline {
    pub fn new(data: ProjectionData) -> Self {
        let args = data.args;

        let t_prp: SMatrix<f64, 4, 4> = SMatrix::from_rows(&[
            RowVector4::new(1.0, 0.0, 0.0, -args.x),
            RowVector4::new(0.0, 1.0, 0.0, -args.y),
            RowVector4::new(0.0, 0.0, 1.0, -args.z),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        let s: SMatrix<f64, 4, 4> = SMatrix::from_rows(&[
            RowVector4::new(
                (2.0 * args.z) / ((args.U - args.u) * (args.z - args.B)),
                0.0,
                0.0,
                0.0,
            ),
            RowVector4::new(
                0.0,
                (2.0 * args.z) / ((args.V - args.v) * (args.z - args.B)),
                0.0,
                0.0,
            ),
            RowVector4::new(0.0, 0.0, 1.0 / (args.z - args.B), 0.0),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        let normal: SMatrix<f64, 4, 4> = s * (data.sh * (t_prp * (data.r * data.t_vrp)));
        let d: f64 = args.z / (args.B - args.z);

        PerspectivePipeline { normal, d }
    }

    pub fn normal(self) -> SMatrix<f64, 4, 4> {
        self.normal
    }

    pub fn normalize(self, model: Model) -> Vec<Vec<Vec<f64>>> {
        let mut v_out: Vec<Vec<Vec<f64>>> = vec![];
        for polygon in model.polygons.iter() {
            let mut vertices: Vec<Vec<f64>> = vec![];
            for vertex in polygon.vertices.iter().rev() {
                let v_h = self.normal * vertex.clone().to_vector();
                let x = v_h[0] / (v_h[2] / self.d);
                let y = v_h[1] / (v_h[2] / self.d);
                let v = vec![x, y];
                vertices.push(v);
            }
            vertices.push(vertices[0].clone());
            v_out.push(vertices);
        }
        v_out
    }
}
