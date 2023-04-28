/// Not needed in airbased scenario


use nalgebra as na;

#[derive(Debug)]
pub struct Face {
    pub face_id: usize,
    pub position: na::Point3<f64>,
    pub normalized_normal: na::Vector3<f64>,
    pub distance_to_center: f64,
}

impl Face {
    pub fn new(
        face_id: usize,
        position: na::Point3<f64>,
        normalized_normal: na::Vector3<f64>,
        distance_to_center: f64,
    ) -> Face {
        Face {
            face_id,
            position,
            normalized_normal,
            distance_to_center,
        }
    }

    pub fn point_distance_to_face(self, position: na::Point3<f64>) -> f64 {
        let pos_diff = position - self.position;
        self.normalized_normal.dot(&pos_diff)
    }
}
