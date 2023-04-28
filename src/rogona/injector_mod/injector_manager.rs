/* INJECTOR MANAGER FOR SPRAYNOZZLES */

// TODO [Future Work] Every SprayNozzle should be able to be changed to an injector trait obj

use crate::rogona::{
    injector_mod::injector::SprayNozzle,
    rg_attributes::pgtraits::{KernelCompNames, PGComponent},
};

use std::collections::{hash_map::ValuesMut, HashMap};


 #[derive(Debug)]
pub struct InjectorManager {
    inj_arr: HashMap<usize, Box<SprayNozzle>>, // find Injectors by ID - store as single copy on the heap
    total_counter: usize,                      // determines Injector ID
}

// TODO [Future work] configure a trait for all managers via the PGComponent Trait or similar
impl PGComponent for InjectorManager {
    type Comp = SprayNozzle;

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::InjectorManager
    }
}

impl InjectorManager {
    pub fn new() -> InjectorManager {
        InjectorManager {
			inj_arr: HashMap::new(),
			total_counter: 0,
		}
    }

	pub fn get_injector(&mut self, id: usize) -> Option<&mut Box<SprayNozzle>> {
		self.inj_arr.get_mut(&id)
	}

    pub fn get_all_injectors(&mut self) -> ValuesMut<'_, usize, Box<SprayNozzle>> {
        //returns consumable Iterator
        self.inj_arr.values_mut()
    }

	//Destruction might be added later
	pub fn apply_changes(&mut self, mut add_arr: Vec<Box<SprayNozzle>>){
		loop{
            match add_arr.pop(){
                Some(mut inj) => {
                    inj.set_id(self.total_counter);
                    self.inj_arr.insert(self.total_counter, inj);

                    self.total_counter += 1;    //move up if you want to start numbering the injectors at 1
                },
                None => break
            }
        }
	}

}
