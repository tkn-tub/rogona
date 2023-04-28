use std::{str::FromStr, time::Instant};
use std::env;

use log::info;

use crate::molcom_recon::evaluation;

use super::{threshold_impl, k_means_impl};

#[derive(Debug)]
enum ReconstructionImplementation {
    Threshold,
    KMeans
}



impl FromStr for ReconstructionImplementation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Threshold" => Ok(ReconstructionImplementation::Threshold),
            "KMeans" => Ok(ReconstructionImplementation::KMeans),
            _ => Err(String::from("Invalid Reconstruction Implementation chosen.")),
        }
    }
}



pub fn main() {
    let args: Vec<String> = env::args().collect();

    let setup = match ReconstructionImplementation::from_str(&args[3]) {
        Ok(p) => p,
        Err(msg) => panic!("{}",msg),
    };

    info!(target: "Setup", "Reconstruction Phase: Setup: {:#?}", setup);
    let t_start = Instant::now();

    //let livs = reconstruction::get_liv_synced(&args[2]);
    let eval_method;
    match setup {
        ReconstructionImplementation::Threshold => eval_method = threshold_impl::reconstruct(&args[2], &args[4]),
        ReconstructionImplementation::KMeans => todo!(),
    }
    let t_recon = t_start.elapsed();
    info!(target: "Simulation Times", "Reconstruction Time: {}", t_recon.as_secs_f64());

    let t_start = Instant::now();
    //TODO [Future Work] make it compatible to KMeans Recon
    evaluation::evaluate(&args[2], &args[4], eval_method);
    let t_eval = t_start.elapsed();
    info!(target: "Simulation Times", "Evaluation Time: {}", t_eval.as_secs_f64());

}