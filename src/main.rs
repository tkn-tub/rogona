use std::env;
use std::str::FromStr;

use rogona_ab_molcom::{config_yaml, Mode};
use rogona_ab_molcom::rogona::rogona_main;
use rogona_ab_molcom::molcom_recon::molcom_recon_main;


fn main() {

    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mode = match Mode::from_str(&args[1]) {
        Ok(m) => m,
        Err(_) => panic!("Impossible Mode return. Investigate!"),
    };

    /* Command Line Arguments for simulation
    [1] "Air" 
    [2] simulation config
    [3] "Threshold"
    [4] recon config
    */
    if mode == Mode::OneFile {
        if !env::var("RECON_ONLY").is_ok(){
            rogona_main::main();
        }
        if !env::var("SIM_ONLY").is_ok() {
            molcom_recon_main::main(); 
        }
    } else {
        config_yaml::main(&mode);
    }


}
