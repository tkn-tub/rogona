/* SENSOR MANAGER */

use crate::rogona::{
    molecule_mod::molecule::Molecule,
    rg_attributes::pgtraits::{KernelCompNames, PGComponent, PGSensor},
    sensor_mod::{
        sensor_camera::SensorCamera, sensor_counting::SensorCounting,
        sensor_destructing::SensorDestructing,
    },
};

use core::time;
use std::slice::Iter;


#[derive(Debug)]
pub struct SensorManager {
    destructing: Vec<SensorDestructing>,
    counting: Vec<SensorCounting>,
    camera: Option<SensorCamera>,
}

impl PGComponent for SensorManager {
    type Comp = SensorCamera; //doesn't make much sense yet but can't be a trait. maybe that can be changed in the future

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::SensorManager
    }

    fn attach_component(&mut self, comp: Self::Comp) {
        panic!("Not implemented for SensorManager");
    }
}

impl SensorManager {
    pub fn new() -> SensorManager {
        SensorManager {
            destructing: vec![],
            counting: vec![],
            camera: None,
        }
    }

    pub fn attach_camera(&mut self, cam: SensorCamera) {
        self.camera = Some(cam);
    }

    pub fn get_molecule_count(&self) -> usize {
        self.counting.iter().map(|s| s.get_count()).sum() //extend later to pretty print with Counting ID's
    }

    pub fn add_sensor_destructing(&mut self, s: SensorDestructing) {
        self.destructing.push(s);
    }

    pub fn add_sensor_counting(&mut self, s: SensorCounting) {
        self.counting.push(s);
    }

    pub fn cam_mol(&mut self, mol: &Box<Molecule>) -> bool {
        match &mut self.camera {
            Some(cam) => cam.check(mol.get_position()),
            None => false,
        }
    }

    pub fn push_liv(&mut self) {
        match &mut self.camera {
            Some(cam) => cam.push_liv(),
            None => (),
        }
    }

    pub fn get_liv_arr(&self) -> Vec<f64> {
        match &self.camera {
            Some(cam) => cam.get_liv_arr(),
            None => vec![],
        }
    }

    pub fn destruct_mol(&self, mol: &Box<Molecule>) -> bool {
        for sensor in self.destructing.iter() {
            if sensor.destruct(mol) {
                return true;
            }
        }
        false

        /* TODO [Future Work fluid-based]
        HashMap or Array based on cell_id
        lets you check if there is a sensor_destructing in charge of that cell */
    }

    pub fn count_mol(&mut self, mol: &Box<Molecule>, sim_time: &time::Duration) {
        for sensor in self.counting.iter_mut() {
            if sensor.count(mol, sim_time) {
                break;
            }
        }

        /* TODO [Future Work fluid-based]
        HashMap/Array based on cell_id
        then the specific sensor can be chosen */
    }

    pub fn get_sensor_counting(&self) -> Iter<SensorCounting> {
        self.counting.iter()
    }
}
