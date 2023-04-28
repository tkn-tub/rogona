/* For this application a spray nozzle is sufficient.
It will be implemented first and in later adaptations a general injector struct can be introduced. */

// Does something in Notification Stage SPAWNING

use log::debug;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::time;

use nalgebra::{Point3, Vector3};
use std::f64::consts::PI;

use crate::rogona::{molecule_mod::molecule::Molecule, rg_attributes::pgtraits::PGObj};

/** VALUES FROM BHATTACHARJEE ET AL
 *  
 * velocity = 12.82 meter/second
 * velocity_sigma = 3 m/s 
 * distribution_sigma = 1.55
 *
 * inj_amount_per_time = 0.54 liter/minute (0,009 liter/second)
 * shutting_time = minimum 30ms (0.03 s)
 * liter_molecule_conversion = ?? (molecules/liter)
 * ======>
 * inj_amount_base (_over_shutting_time) = inj_amount_per_time * shutting_time * liter_molecule_conversion
 *
 */

#[derive(Debug)]
pub struct SprayNozzle {
    // find in Injector Manager and respective Modulation and Bitstreamgenerator
    id: usize,
    // Geometry and Position
    // TODO [Future Work]: put transformation instead
    translation: Vector3<f64>,
    rotation: Vector3<f64>,
    scaling: Vector3<f64>,
    neg_y: bool,
    // Functionality hard
    inj_amount_base: i64, //injection_amount per symbol '1'
    velocity: f64,
    velocity_sigma: f64,
    distribution_sigma: f64,
    seed: Option<f64>, // ?
    base_delta_time: time::Duration,
    shutter_time: time::Duration,
    // runtime-factors
    floor: bool, // alternating between flooring and ceiling the inj_amount_step
    turned_on: bool,
    burst_on: bool,
    inj_amount_step: u32,
    step_delta_time: time::Duration,
    t_begin: time::Duration,
    t_end: time::Duration,
    inj_amount_step_2: u32,
    step_delta_time_2: time::Duration,
    t_begin_2: time::Duration,
    t_end_2: time::Duration,
    second_spray: bool,
}

impl PGObj for SprayNozzle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// the molecules have to be sent to the end! when they don't completely fill the base_delta_time

impl SprayNozzle {
    pub fn new(
        id: usize,
        translation: Vector3<f64>,
        rotation: Vector3<f64>,
        scaling: Vector3<f64>,
        neg_y: bool,
        inj_amount_base: i64,
        velocity: f64,
        velocity_sigma: f64,
        distribution_sigma: f64,
        seed: Option<f64>,
        base_delta_time: time::Duration,
        shutter_time: time::Duration,
    ) -> SprayNozzle {
        SprayNozzle {
            id,
            translation,
            rotation,
            scaling,
            neg_y,
            inj_amount_base,
            velocity,
            velocity_sigma,
            distribution_sigma,
            seed,
            base_delta_time,
            shutter_time,
            floor: false,
            turned_on: false,
            burst_on: false,
            inj_amount_step: 0,
            step_delta_time: time::Duration::new(0, 0),
            t_begin: time::Duration::new(0, 0),
            t_end: time::Duration::new(0, 0),
            inj_amount_step_2: 0,
            step_delta_time_2: time::Duration::new(0, 0),
            t_begin_2: time::Duration::new(0, 0),
            t_end_2: time::Duration::new(0, 0),
            second_spray: false,
        }
    }

    // spawn from "already far away from sprayer" to "close to the sprayer"
    pub fn spawn(&self) -> Vec<Box<Molecule>> {
        if self.turned_on && self.inj_amount_step > 0 {
            let mut spawn_array: Vec<Box<Molecule>> = vec![];

            let mut delta_time = self.t_begin;

            while delta_time < self.t_end {
                
                spawn_array.push(self.molecule(self.base_delta_time - delta_time));

                delta_time += self.step_delta_time;

            }

            if self.second_spray && self.inj_amount_step_2 > 0 {
                delta_time = self.t_begin_2;

                while delta_time <= self.t_end_2 {
                    spawn_array.push(self.molecule(self.base_delta_time - delta_time));

                    delta_time += self.step_delta_time_2;
                }
            }
            spawn_array
        } else {
            vec![]
        }
    } // injector_manager or simulation kernel can handle it from here.

    // !
    // Directly translated from Pogona except *
    fn molecule(&self, travel_time: time::Duration) -> Box<Molecule> {
        let mut rng = rand::thread_rng();

        // Check Transformation.py

        let used_velocity = Normal::new(self.velocity, self.velocity_sigma)
            .unwrap()
            .sample(&mut rng); // https://rust-random.github.io/rand/rand_distr/struct.Normal.html

        // Uniformly distributed angle between 0 and 2*pi by which
        // the new velocity vector will be roated around the y-axis:
        let used_3d_angle = rng.gen_range(0.0..1.0) * 2.0 * PI;

        // Normally distributed angle by which the new velocity
        // vector will be rotated around the x- and z-axis:
        let used_distribution = Normal::new(0.0, self.distribution_sigma)
            .unwrap()
            .sample(&mut rng)
            * (PI / 180.0); // conversion deg to rad

        let velocity_x = used_velocity * used_distribution.sin() * used_3d_angle.sin();

        let mut velocity_y = used_velocity * used_distribution.cos();

        let velocity_z = used_velocity * used_distribution.sin() * used_3d_angle.cos();

        // ! FOR NOW JUST EITHER positive or negative y therefore
        if self.neg_y {
            velocity_y *= -1.0;
        }

        let velocity = Vector3::new(velocity_x, velocity_y, velocity_z);

        // *
        let position = self.translation + velocity.scale(travel_time.as_secs_f64());

        Box::new(Molecule::new(Point3::from(position), velocity, None))
    }

    pub fn turn_off(&mut self, version: i8) {
        match version {
            2 => self.second_spray = false,
            _ => self.turned_on = false,
        };
    }

    pub fn get_shutting_time(&self) -> time::Duration {
        self.shutter_time
    }

    pub fn get_inj_amount_base(&self) -> i64 {
        self.inj_amount_base
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }


    // Accessed bei modulation
    pub fn set_next_spawn(
        &mut self,
        inj_fraction: f64,
        t_begin: time::Duration,
        t_end: time::Duration,
        version: i8,
    ) {
        self.turned_on = true;
        match version {
            1 => {
                match self.floor {
                    true => {
                        self.inj_amount_step =
                            (self.inj_amount_base as f64 * inj_fraction).floor() as u32;
                    }
                    false => {
                        self.inj_amount_step =
                            (self.inj_amount_base as f64 * inj_fraction).ceil() as u32;
                    }
                }
                self.floor = !self.floor; //toggle floor attribute
                if self.inj_amount_step == 0 {
                    self.step_delta_time = self.base_delta_time;
                } else {
                    self.step_delta_time = (t_end - t_begin) / self.inj_amount_step;
                }
                self.t_begin = t_begin;
                self.t_end = t_end;
                self.second_spray = false;
            }
            2 => {
                match self.floor {
                    true => {
                        self.inj_amount_step_2 =
                            (self.inj_amount_base as f64 * inj_fraction).floor() as u32;
                    }
                    false => {
                        self.inj_amount_step =
                            (self.inj_amount_base as f64 * inj_fraction).ceil() as u32;
                    }
                }
                self.floor = !self.floor; //toggle floor attribute
                if self.inj_amount_step_2 == 0 {
                    self.step_delta_time_2 = self.base_delta_time;
                } else {
                    self.step_delta_time_2 = (t_end - t_begin) / self.inj_amount_step_2;
                }
                self.t_begin_2 = t_begin;
                self.t_end_2 = t_end;
                self.second_spray = true;
            }
            _ => {
                self.turn_off(version);
                eprintln!("No behavior defined. Please check whether base_delta_time <= t_sym (bit_duration)")
            }
        }
    }
}



#[cfg(test)]

mod tests {
    use super::*;

    fn example_injector(
        inj_amount_base: i64,
        base_delta_time: time::Duration,
        t_shut: time::Duration,
        neg_y: bool,
    ) -> SprayNozzle {
        SprayNozzle {
            id: 1,
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scaling: Vector3::new(1.0, 1.0, 1.0),
            neg_y,
            inj_amount_base,
            velocity: 5.0,
            velocity_sigma: 0.0,
            distribution_sigma: 0.0,
            seed: None,
            base_delta_time,
            shutter_time: t_shut,
            floor: true,
            turned_on: false,
            burst_on: false,
            inj_amount_step: 0,
            step_delta_time: time::Duration::new(0, 0),
            t_begin: time::Duration::new(0, 0),
            t_end: time::Duration::new(0, 0),
            inj_amount_step_2: 0,
            step_delta_time_2: time::Duration::new(0, 0),
            t_begin_2: time::Duration::new(0, 0),
            t_end_2: time::Duration::new(0, 0),
            second_spray: false,
        }
    }

    #[test]
    fn just_try_sth() {
        let inj_amount_base = 10;
        let base_delta_time = time::Duration::new(1, 0);
        let t_shut = time::Duration::new(2, 0);
        let mut inj = example_injector(inj_amount_base, base_delta_time, t_shut, false);

        let t_begin = time::Duration::new(0, 0);
        let t_end = base_delta_time;
        let inj_fraction = (t_end - t_begin).as_nanos() as f64 / t_shut.as_nanos() as f64;
        inj.set_next_spawn(inj_fraction, t_begin, t_end, 1);
        let mol_arr = inj.spawn();
        let mut c = 0;
        let debug_arr: Vec<(i32, Point3<f64>)> = mol_arr
            .iter()
            .map(|mol| {
                c += 1;
                (c, mol.get_position())
            })
            .collect();
        println!("{:#?}", debug_arr);
    }
}
