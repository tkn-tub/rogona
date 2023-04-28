use crate::rogona::init;
use crate::rogona::simulation_kernel::SimulationKernel;

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;
use std::env;

extern crate log;
use log::{debug, info};
/** To turn Logging on you can use e.g. RUST_LOG=trace and then run the application. See https://docs.rs/env_logger/latest/env_logger/ for further options.
 *  In VSCode use the launch.json by using CodeLLDB as a Debugging extension 
 * 
 */

use std::time::Instant;

#[derive(PartialEq, Debug)]
enum Piece {
    AirTube,
    Tube,
    YPiece,
    Other
}


impl FromStr for Piece {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tube" => Ok(Piece::Tube),
            "YPiece" => Ok(Piece::YPiece),
            "Air" => Ok(Piece::AirTube),
            _ => Err(String::from("Invalid Object Type.")),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Liv {
    liv_path: String
}

pub fn main() {

    let args: Vec<String> = env::args().collect();
    
    let setup = match Piece::from_str(&args[1]) {
        Ok(p) => p,
        Err(msg) => panic!("{}",msg),
    }; 
    let print_on = false;
    let config_path = &args[2];
    
    info!("Rogona Simulator - Testscenario for Airbased Molecular Communication");

    let t_start = Instant::now();

    info!(target: "Simulation Times", "Simulator started");

    

    let mut simulation_kernel: SimulationKernel;

    debug!(target: "Setup", "Simulated Piece: {:?}", setup);
    if setup == Piece::Tube {
        simulation_kernel = init::init_tube();
    } else if setup == Piece::YPiece {
        simulation_kernel = init::init_ypiece();
    } else if setup == Piece::AirTube {
        simulation_kernel = init::init_air_tube(&config_path);
    } else {
        simulation_kernel = SimulationKernel::new_empty();
    }

    let t_init = t_start.elapsed();
    info!(target: "Simulation Times", "Initialization complete after {:4}s", t_init.as_secs_f64());


    let t_simstart = Instant::now();

    if setup != Piece::Other {
        simulation_kernel.simulation_loop();
    }

    let t_sim = t_simstart.elapsed();

    info!(target: "Simulation Times", "Simulation complete after {:4}s", t_sim.as_secs_f64());


    let s = fs::read_to_string(&args[2]).expect("Config not found");
    let liv: Liv = serde_yaml::from_str(&s).unwrap();

    let mut liv_out = File::create(&liv.liv_path).expect("unable to create liv file");
    for liv in simulation_kernel.get_sensor_manager().get_liv_arr() {
        write!(&mut liv_out, "{};", liv).expect("unable to write into liv file");
    }

    // not used in air-based scenario
    if print_on {
        let t_printstart = Instant::now();

        let mut sc_out =
            File::create("sensor_counting.txt").expect("unable to create sensor_counting.txt");
        write!(
            &mut sc_out,
            "Molecules counted: {} in {} sensors\n",
            simulation_kernel.get_molecule_count(),
            simulation_kernel
                .get_sensor_manager()
                .get_sensor_counting()
                .len()
        )
        .expect("unable to write into sensor_counting.txt");
        for sc in simulation_kernel.get_sensor_manager().get_sensor_counting() {
            write!(&mut sc_out, "{}", sc).expect("unable to write into sensor_counting.txt\n");
        }

        /* let mut molpath = File::create("molpaths.txt").expect("unable to create molpath.txt");
        write!(
            &mut molpath,
            "{}",
            simulation_kernel.get_movement_predictor().get
        )
        .expect("unable to write into molpath.txt"); */

        info!(target: "Simulation Times",
            "Printing done in \t{:4}s",
            t_printstart.elapsed().as_secs_f64()
        );
    }

    info!(target: "Simulation Times",
        "Whole simulation process done after {:4}s",
        t_start.elapsed().as_secs_f64()
    );

}

