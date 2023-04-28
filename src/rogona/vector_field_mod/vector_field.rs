/// Not needed in airbased scenario


use crate::rogona::vector_field_mod::face::Face;
use core::slice::Iter;
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

#[derive(Debug)]
pub struct VectorField {
    pub cell_centres: Vec<Point3<f64>>,
    pub flow: Vec<Vector3<f64>>,
    pub is_boundary: Vec<bool>,
    pub boundary_faces: HashMap<usize, Face>,
}

impl VectorField {
    pub fn get_cell_centers(&self) -> Iter<Point3<f64>> {
        self.cell_centres.iter()
    }

    pub fn get_flow(&self, cell_id: usize) -> Vector3<f64> {
        self.flow[cell_id]
    }
}
