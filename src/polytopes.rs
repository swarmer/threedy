use std::fmt::Debug;


pub type Point3d = (f64, f64, f64);
pub type Edge = (Point3d, Point3d);


pub trait Polytope : Debug {
    fn edges(&self) -> Vec<Edge>;
}


#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    pub center: Point3d,
    pub height: f64,
}

impl Polytope for Cube {
    fn edges(&self) -> Vec<Edge> {
        let step = self.height / 2.0;

        let (center_x, center_y, center_z) = self.center;
        let l1 = (center_x - step, center_y - step, center_z - step);
        let l2 = (center_x - step, center_y + step, center_z - step);
        let l3 = (center_x + step, center_y + step, center_z - step);
        let l4 = (center_x + step, center_y - step, center_z - step);
        let u1 = (center_x - step, center_y - step, center_z + step);
        let u2 = (center_x - step, center_y + step, center_z + step);
        let u3 = (center_x + step, center_y + step, center_z + step);
        let u4 = (center_x + step, center_y - step, center_z + step);

        vec![
            (l1, l2),
            (l2, l3),
            (l3, l4),
            (l1, l4),
            (l1, u1),
            (u1, u2),
            (l2, u2),
            (u2, u3),
            (u3, u4),
            (u1, u4),
            (l3, u3),
            (l4, u4),
        ]
    }
}
