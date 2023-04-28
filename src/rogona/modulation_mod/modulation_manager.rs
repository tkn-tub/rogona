/* MODULATION MANAGER */

// TODO [Future Work] Every Modulation should be able to be changed to an modulation trait obj

use log::{warn, debug};

use crate::rogona::{
    modulation_mod::modulation_ook::ModulationOOK,
    rg_attributes::pgtraits::{KernelCompNames, PGComponent}, 
    injector_mod::injector::SprayNozzle,
};

use std::{collections::{hash_map::ValuesMut, HashMap}, time};


 #[derive(Debug)]
pub struct ModulationManager {
    mod_arr: HashMap<usize, Box<ModulationOOK>>, // find Modulation Units by ID - store as single copy on the heap
    total_counter: usize,                      // determines Modulation Unit ID
}

// TODO [Future work] configure a trait for all managers via the PGComponent Trait or similar
impl PGComponent for ModulationManager {
    type Comp = ModulationOOK;

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::ModulationManager
    }
}

impl ModulationManager {
    pub fn new() -> ModulationManager {
        ModulationManager {
			mod_arr: HashMap::new(),
			total_counter: 0,
		}
    }

	pub fn get_modulator(&mut self, id: usize) -> Option<&mut Box<ModulationOOK>> {
		self.mod_arr.get_mut(&id)
	}

    pub fn get_all_modulators(&mut self) -> ValuesMut<'_, usize, Box<ModulationOOK>> {
        //returns consumable Iterator
        self.mod_arr.values_mut()
    }

	//Destruction might be added later
	pub fn apply_changes(&mut self, mut add_arr: Vec<Box<ModulationOOK>>){
		loop{
            match add_arr.pop(){
                Some(mut modulation) => {
                    modulation.set_id(self.total_counter);
                    self.mod_arr.insert(self.total_counter, modulation);

                    self.total_counter += 1;    //move up if you want to start numbering the MODs at 1
                },
                None => break
            }
        }
	}

    // activate the modulation instance with the same ID
    pub fn modulate(&mut self, sim_time: &time::Duration, inj: &mut Box<SprayNozzle>) {
        
        let modu = match self.mod_arr.get_mut(&inj.get_id()) {
            Some(modulation) => modulation,
            None => {warn!("This Injector doesn't habe an attached Modulation (ID: {}", inj.get_id()); return;}
        };
        modu.modulate(inj, *sim_time);
        
    }

}