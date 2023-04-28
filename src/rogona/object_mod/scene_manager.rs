/// Not needed in airbased scenario


extern crate nalgebra as na;
use na::{Point3, Vector3};

use crate::rogona::{
    object_mod::{object::TObject, tube::Tube},
    rg_attributes::pgtraits::{Interpolation, KernelCompNames, PGComponent},
    molecule_mod::molecule::Molecule
};

#[derive(Debug)]
pub struct SceneManager {
    //name : KernelCompNames,
    objects: Vec<Tube>, //change to trait object later
    total_counter: u64, //for setting object_ids
}

impl PGComponent for SceneManager {
    type Comp = Tube; //not yet generic. Same issue as sensor_manager

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::SceneManager
    }
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            //name : KernelCompNames::SceneManager,
            objects: Vec::new(),
            total_counter: 0,
        }
    }

    // TODO: [Future Work] change to trait object in future versions
    pub fn add_object(&mut self, mut t: Tube) {
        t.object_id = Some(self.total_counter);
        self.total_counter += 1;
        self.objects.push(t);
    }

    pub fn get_flow_by_position(&self, global_pos: Point3<f64>, obj_id: u64, int_method: Interpolation) -> Vector3<f64> {
        self.objects[obj_id as usize].get_flow_by_position(global_pos, int_method)
    }
    pub fn get_cell_id_by_position(&self, global_pos: Point3<f64>, obj_id: u64) -> Option<usize> {
        self.objects[obj_id as usize].get_cell_id_by_position(global_pos)
    }
}
