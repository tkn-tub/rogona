use std::time;

use log::{trace};

use crate::rogona::{
    injector_mod::{injector::SprayNozzle, injector_manager::InjectorManager}, 
    rg_attributes::{pgtraits::PGObj, stages::SimStages},
};

// implement later with Traitobject injector
#[derive(Clone, Copy, Debug)]
pub struct ModulationOOK {
    id: usize,
    pub current_bit: u8,
    next_bit: Option<u8>, // in case the next bits transmission already starts
    start_time: time::Duration,
    symbol_count: u64,     // N
    t_sym: time::Duration, // symbol_duration = Injector.t_shut + t_pause and per definition: t_sym >= t_shut
    base_delta_time: time::Duration,
    transmission_lock: bool, // true: a bit is still being transmitted, false: bg can give next bit to Modulation
}

impl PGObj for ModulationOOK {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ModulationOOK {
    pub fn new(
        id: usize,
        start_time: time::Duration,
        t_sym: time::Duration,
        base_delta_time: time::Duration,
    ) -> ModulationOOK {
        ModulationOOK {
            id,
            current_bit: 0,
            next_bit: None,
            start_time,
            symbol_count: 0,
            t_sym,
            base_delta_time,
            transmission_lock: false,
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    // refer to thesis UML chart
    pub fn modulate(&mut self, inj: &mut Box<SprayNozzle>, t_sim: time::Duration) -> Option<u8> {
        
        let return_value: Option<u8> = None;

        if self.symbol_count <= 0 {
            trace!("No symbol sequenced yet");
            return return_value;
        }
        
        let mut version = 1;
        loop {            
            // start_time is adjusted to the next '1' over every delta_sim_step
            if self.current_bit != 1 {
                inj.turn_off(version);
                self.transmission_lock = false;

                break;
            } else {
                let t_start = self.start_time
                + time::Duration::from_secs_f64(
                    (self.symbol_count - 1) as f64 * self.t_sym.as_secs_f64(),
                ); // t_start = start_time + (N-1) * t_sym


                //symbol == 1
                if t_sim <= t_start {
                    inj.turn_off(version);
                    break;
                } else {
                    // t_sim > t_start; spray sth
                    
                    let t_diff = t_sim - t_start;
                    let t_shut = inj.get_shutting_time();
                    let inj_amount_base = inj.get_inj_amount_base();
                    let t_begin;
                    let t_end;
                    

                    if t_diff.as_secs_f64() - self.base_delta_time.as_secs_f64() <= 0.0 {   //TODO warum nicht tdiff<= base delta ? test!
                        // spraying started during this time step
                        if t_diff < t_shut {
                            // spraying continues after this time-step
                            //case A: fraction of injection_amount close to the sprayer
                            //debug!("Case A");
                            
                            t_begin = self.base_delta_time - t_diff;

                            t_end = self.base_delta_time;
                            
                        } else {
                            // spraying ends during this time step
                            //case B: complete injection_amount during this time step
                            //debug!("Case B");
                            
                            t_begin = self.base_delta_time - t_diff;
                            t_end = self.base_delta_time + t_shut - t_diff;
                            self.transmission_lock = false;
                            
                        }
                    } else {
                        
                        // spraying already ongoing during this time step
                        if t_diff < t_shut {
                            // spraying continues after this time-step
                            //case C: fraction of injection_amount during whole time_step
                            //debug!("Case C");
                            
                            t_begin = time::Duration::new(0, 0);
                            t_end = self.base_delta_time;
                            
                        } else {
                            // spraying ends during this time step
                            //case D: last fraction of injection_amount far from the sprayer
                            //debug!("Case D");
                            
                            t_begin = time::Duration::new(0, 0);
                            t_end = self.base_delta_time + t_shut - t_diff;

                            self.transmission_lock = false;
                        }
                        
                    }
                    

                    let inj_fraction = (t_end - t_begin).as_secs_f64() / t_shut.as_secs_f64();

                    inj.set_next_spawn(inj_fraction, t_begin, t_end, version);

                    //debug!("t_diff: {} , t_sym: {}", t_diff.as_secs_f64(), self.t_sym.as_secs_f64());
                    if t_diff >= self.t_sym {
                        // case E: new symbol already starts
                        self.current_bit = self
                            .next_bit
                            .expect("no next bit was provided. maybe delta_sim_time too big?");
                        self.next_bit = None;
                        self.increase_symbol_count();

                        version += 1;
                        continue;
                    }

                    break;
                }
            }
        }
        
        return_value
    }

    pub fn set_current_bit(&mut self, bit: u8) {
        self.current_bit = bit;
    }

    pub fn get_current_bit(&self) -> u8 {
        self.current_bit
    }

    pub fn set_next_bit(&mut self, bit: u8) {
        self.next_bit = Some(bit);
    }

    pub fn get_next_bit(&self) -> Option<u8> {
        self.next_bit
    }

    pub fn increase_symbol_count(&mut self) {
        self.symbol_count += 1;
    }

    pub fn lock_transmission(&mut self) {
        self.transmission_lock = true;
    }

    pub fn transmission_locked(&self) -> bool {
        self.transmission_lock
    }
}
