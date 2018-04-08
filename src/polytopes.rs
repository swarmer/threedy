use std::fmt::Debug;


pub type Vector3d = (f64, f64, f64);
pub type Edge = (Vector3d, Vector3d);


pub trait Polytope : Debug {
    fn get_edges(&self) -> Vec<Edge>;

    fn get_scale(&self) -> f64;
    fn set_scale(&mut self, scale: f64);

    fn get_rotation(&self) -> Vector3d;
    fn set_rotation(&mut self, rotation: Vector3d);

    fn get_offset(&self) -> Vector3d;
    fn set_offset(&mut self, offset: Vector3d);

    fn rotate(&mut self, angles: Vector3d) {
        let (angle_x, angle_y, angle_z) = angles;
        let (mut x, mut y, mut z) = self.get_rotation();

        x += angle_x;
        y += angle_y;
        z += angle_z;

        self.set_rotation((x, y, z));
    }

    fn shift(&mut self, offset: Vector3d) {
        let (offset_x, offset_y, offset_z) = offset;
        let (mut x, mut y, mut z) = self.get_offset();

        x += offset_x;
        y += offset_y;
        z += offset_z;

        self.set_offset((x, y, z));
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    pub offset: Vector3d,
    pub rotation: Vector3d,
    pub scale: f64,
}

impl Polytope for Cube {
    fn get_edges(&self) -> Vec<Edge> {
        let l1 = (-1.0, -1.0, -1.0);
        let l2 = (-1.0, 1.0, -1.0);
        let l3 = (1.0, 1.0, -1.0);
        let l4 = (1.0, -1.0, -1.0);
        let u1 = (-1.0, -1.0, 1.0);
        let u2 = (-1.0, 1.0, 1.0);
        let u3 = (1.0, 1.0, 1.0);
        let u4 = (1.0, -1.0, 1.0);

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

    fn get_scale(&self) -> f64 { self.scale }
    fn set_scale(&mut self, scale: f64) { self.scale = scale; }

    fn get_rotation(&self) -> Vector3d { self.rotation }
    fn set_rotation(&mut self, rotation: Vector3d) { self.rotation = rotation; }

    fn get_offset(&self) -> (f64, f64, f64) { self.offset }
    fn set_offset(&mut self, offset: Vector3d) { self.offset = offset; }
}
