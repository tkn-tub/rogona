use nalgebra::{Point3, Vector3};
use std::fmt;

use crate::rogona::rg_attributes::pgtraits::PGObj;

#[derive(Debug, PartialEq)]
pub struct Molecule {
    id: Option<usize>,
    position: Point3<f64>,
    velocity: Vector3<f64>,     //~~ for later versions
    object_id: Option<u64>,
    cell_id: Option<usize>,
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match self.id {
            Some(v) => format!("{}", v),
            None => format!("not initialized"),
        };
        let object_id = match self.object_id {
            Some(v) => format!("{}", v),
            None => format!("not set"),
        };
        write!(
            f,
            "Molecule {} at {} in Object {}",
            id, self.position, object_id
        )
    }
}

impl PGObj for Molecule {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Molecule {

    /// Creates a new Molecule in a specific object with a starting position and velocity.
    pub fn new(position: Point3<f64>, velocity: Vector3<f64>, object_id: Option<u64>) -> Molecule {
        Molecule {
            position,
            velocity,
            object_id,
            cell_id: None, //set by movement_predictor
            id: None,      //set by molecule_manager
        }
    }

    /// ! deprecated
    pub fn copy_as_new(&self, id: Option<usize>) -> Molecule {
        Molecule { id, ..*self }
    }

    /// ! deprecated
    pub fn deepcopy(&self) -> Molecule {
        Molecule { ..*self }
    }

    /// * Getters and Setters

    pub fn get_id(&self) -> Option<usize> {
        self.id
    }

    pub fn get_position(&self) -> Point3<f64> {
        self.position
    }
    pub fn get_velocity(&self) -> Vector3<f64> {
        self.velocity
    }
    pub fn get_object_id(&self) -> Option<u64> {
        self.object_id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    pub fn set_position(&mut self, pos: Point3<f64>) {
        self.position = pos;
    }

    pub fn set_velocity(&mut self, velocity: Vector3<f64>) {
        self.velocity = velocity;
    }

    pub fn set_object_id(&mut self, obj_id: Option<u64>) {
        self.object_id = obj_id;
    }

    pub fn set_cell_id(&mut self, cell_id: Option<usize>) {
        self.cell_id = cell_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_example_molecule() -> Molecule {
        let pos = Point3::new(1.0f64, 2.0, 3.0);
        let vel = Vector3::new(3.0f64, 4.0, 5.0);
        Molecule::new(pos, vel, Some(30))
    }

    #[test]
    fn normal_print_valid_object() {
        let m = build_example_molecule();
        assert_eq!(
            format!("Testing prints for Molecules: {}", m),
            "Testing prints for Molecules: Molecule not initialized at {1, 2, 3} in Object 30"
        );
    }

    #[test]
    fn normal_print_invalid_object() {
        let pos = Point3::new(1.0f64, 2.0, 3.0);
        let vel = Vector3::new(3.0f64, 4.0, 5.0);
        let m = Molecule::new(pos, vel, None);
        assert_eq!(
            format!("Testing prints for Molecules: {}", m),
            "Testing prints for Molecules: Molecule not initialized at {1, 2, 3} in Object not set"
        );
    }

    #[test]
    fn molecule_copy() {
        let m = build_example_molecule();
        let m2 = m.deepcopy();
        assert_eq!(m, m2);
    }

    #[test]
    fn getter() {
        let m = build_example_molecule();
        print!("{}", m);
        assert_eq!(Point3::new(1.0f64, 2.0, 3.0), m.get_position());
    }

    #[test]
    fn setter() {
        let mut m = build_example_molecule();
        m.set_id(5);
        assert_eq!(m.get_id(), Some(5));
    }
}
