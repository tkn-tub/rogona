// not needed in air-based scenario

use core::time;
use std::collections::HashMap;
use std::fmt;

use crate::rogona::{
    rg_attributes::pgtraits::PGSensor,
    molecule_mod::molecule::Molecule
};

#[derive(Debug)]
pub struct SensorCounting {
    count: i64,
    zmin: f64,
    zmax: f64,
    log: HashMap<usize, time::Duration>,
}

impl PGSensor for SensorCounting {}

impl fmt::Display for SensorCounting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = String::from(format!(
            "Sensor from z = {} to z = {} counted {}\nDetected these molecules (ID, Timestep):\n",
            self.zmin, self.zmax, self.count
        ));
        for (id, ts) in self.log.iter() {
            printer = printer + format!("({}, {:4}s)\n", id, ts.as_secs_f64()).as_str();
        }
        write!(f, "{}", printer)
    }
}

impl SensorCounting {

    // ~~ mapping to cell IDs in future versions
    pub fn new(zmin: f64, zmax: f64) -> SensorCounting {
        SensorCounting {
            count: 0,
            zmin,
            zmax,
            log: HashMap::new(),
        }
    }

    pub fn count(
        &mut self,
        mol: &Box<Molecule>,
        sim_time: &time::Duration,
    ) -> bool {
        let pos = mol.get_position();
        if pos.z > self.zmin && pos.z < self.zmax {
            // ! if try_insert() makes it from nightly_only use that instead.
            // saves last time_step when mol is counted. Only increases counter when mol hasn't been counted before
            match self.log.insert(mol.get_id().unwrap(), sim_time.clone()) {
                Some(_) => (),
                None => {
                    self.count += 1;
                }
            };
            true
        } else {
            false
        }
    }

    pub fn get_count(&self) -> usize {
        self.count as usize
    }
}
