/* BITSTREAMGENERATOR MANAGER */
// TODO [Future work] configure a trait for all managers via the PGComponent Trait

use log::debug;

use crate::rogona::{
    bitstream_generator_mod::bitstreamgenerator::Bitstreamgenerator,
    modulation_mod::modulation_ook::ModulationOOK,
    rg_attributes::pgtraits::{KernelCompNames, PGComponent}
};


use std::{collections::{HashMap}, time};

#[derive(Debug)]
pub struct BitstreamgenManager {
    bg_arr: HashMap<usize, Box<Bitstreamgenerator>>,
    total_counter: usize,
}

// TODO [Future work]
impl PGComponent for BitstreamgenManager {
    type Comp = Bitstreamgenerator;

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::BitstreamgenManager
    }
}

impl BitstreamgenManager {
    pub fn new() -> BitstreamgenManager {
        BitstreamgenManager {
            bg_arr: HashMap::new(),
            total_counter: 0,
        }
    }

    pub fn get_bg(&mut self, id: usize) -> Option<&mut Box<Bitstreamgenerator>> {
		self.bg_arr.get_mut(&id)
	}

    // attaches the bitstreamgenerators in a vector
    pub fn apply_changes(&mut self, mut add_arr: Vec<Box<Bitstreamgenerator>>) {
        loop{
            match add_arr.pop(){
                Some(mut bg) => {
                    bg.set_id(self.total_counter);
                    self.bg_arr.insert(self.total_counter, bg);
                    self.total_counter += 1;    //move up if you want to start numbering the mols at 1
                },
                None => break
            }
        }
    }

    // activate the bitstreamgenerator instance with the same ID
    pub fn bitsequence(&mut self, sim_time: &time::Duration, modu: &mut Box<ModulationOOK>) {
        let bg = match self.bg_arr.get_mut(&modu.get_id()) {
            Some(bitgen) => bitgen,
            None => {eprintln!("This Modulation doesn't have an attached Bitstreamgenerator (ID: {})", modu.get_id()); return}
        };

        bg.bitsequence(sim_time, modu);

    }
}
