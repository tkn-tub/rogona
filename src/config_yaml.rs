use std::fs;
use std::io::Write;
use std::{env, fs::File};

use crate::molcom_recon;
use crate::rogona;
use crate::Mode;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// source: https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step by GordonBGood and Shepmaster (https://stackoverflow.com/a/40168843)
use std::ops::Add;

struct StepRange<T>(T, T, T)
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: PartialOrd,
    T: Clone;

impl<T> Iterator for StepRange<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: PartialOrd,
    T: Clone,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.0 < self.1 {
            let v = self.0.clone();
            self.0 = &v + &self.2;
            Some(v)
        } else {
            None
        }
    }
}
// end of quote

#[derive(Debug, PartialEq)]
enum Variants {
    mps, // absolut molecules per spray
    mpl, // relative molecules per spray
    d,
    dy_cm,   // as small as possible
    dy_pc,   // as small as possible
    sym_no,  // how large is the data collection for learning (circa, likely minimum)
    t_pause, // improves ISI; lowers bitrate
    t_spray, // improves MOL-eye; worse ISI
    w_proj,  // 
    rx_angle,//
    off_f,   // synchronisation differences
    off_n,   // sync
    sim_no,
}

impl FromStr for Variants {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mps" => Ok(Variants::mps),
            "mpl" => Ok(Variants::mpl),
            "d" => Ok(Variants::d),
            "dy_cm" => Ok(Variants::dy_cm),
            "dy_pc" => Ok(Variants::dy_pc),
            "sym_no" => Ok(Variants::sym_no),
            "t_pause" => Ok(Variants::t_pause),
            "t_spray" => Ok(Variants::t_spray),
            "w_proj" => Ok(Variants::w_proj),
            "rx_angle" => Ok(Variants::rx_angle),
            "off_f" => Ok(Variants::off_f),
            "off_n" => Ok(Variants::off_n),
            "sim_no" => Ok(Variants::sim_no),
            _ => Err(String::from("Invalid Attribute.")),
        }
    }
}

pub fn main(mode: &Mode) {
    if *mode == Mode::AddVariants {
        add_variants();
    } else if *mode == Mode::AddLearns {
        add_learns();
    } else if *mode == Mode::AddApplies {
        add_applies();
    } else {
        panic!("This mode does not correspond to a defined behavior")
    }
}

/* Command line args
[1] "addv"
[2] target dir
[3] base configuration
[4] variant attribute {mps, mpl, d, dy_cm, dy_pc, sym_no, w_proj, rx_angle, t_pause, t_spray, off_f, off_n, sim_no}
[5] start
[6] step
[7] end*/
fn add_variants() {
    info!("Starting to build variant configurations.");

    let args: Vec<String> = env::args().collect();
    // assign args
    let target_dir = &args[2];
    let base_config = fs::read_to_string(&args[3]).expect("unable to find/read base config");
    let base_config: rogona::init::ConfigVars = serde_yaml::from_str(&base_config).unwrap();
    let variant_attr = match Variants::from_str(&args[4]) {
        Ok(v) => v,
        Err(msg) => panic!("{}", msg),
    };
    let start: f64 = args[5].parse::<f64>().unwrap_or(0.0);
    let step: f64 = args[6].parse::<f64>().unwrap_or(f64::MAX);
    let end: f64 = args[7].parse::<f64>().unwrap_or(0.0);

    // create sim ConfigVars
    for i in StepRange(start, end, step) {
        let config: rogona::init::ConfigVars;
        let fname;
        match variant_attr {
            Variants::mps => {
                let liv_path = format!("{}_{:?}_{:.0}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    mps: i.floor() as usize,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.0}.yaml", target_dir, variant_attr, i);
            }
            Variants::mpl => {
                let liv_path = format!("{}_{:?}_{:.2}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    lmc: Some(i),
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.2}.yaml", target_dir, variant_attr, i);
            }
            Variants::d => {
                let liv_path = format!("{}_{:?}_{:.2}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    d: i,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.2}.yaml", target_dir, variant_attr, i);
            }
            Variants::dy_cm => {
                let liv_path = format!("{}_{:?}_{:.0}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    dy: i/100.0,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.0}.yaml", target_dir, variant_attr, i);
            }
            Variants::dy_pc => {
                let liv_path = format!("{}_{:?}_{:.1}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    dy: { base_config.d / 2.0 * i / 100.0 },
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.1}.yaml", target_dir, variant_attr, i);
            }
            Variants::sym_no => {
                let liv_path = format!("{}_{:?}_{:.0}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    t_sim: base_config.t_sym * (i + 1.0),
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{}.yaml", target_dir, variant_attr, i);
            }
            Variants::t_pause => {
                let liv_path = format!("{}_{:?}_{:.3}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    t_sym: base_config.t_sp + i,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.3}.yaml", target_dir, variant_attr, i);
            }
            Variants::t_spray => {
                if i > base_config.t_sym {
                    panic!("Spray time is longer than full symbol duration!")
                }
                let liv_path = format!("{}_{:?}_{:.3}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    t_sp: i,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.3}.yaml", target_dir, variant_attr, i);
            }
            Variants::w_proj => {
                let liv_path = format!("{}_{:?}_{:.3}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    cam_w_proj: i,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.3}.yaml", target_dir, variant_attr, i);
            }
            Variants::rx_angle => {
                todo!();
                // intercept theorem.
                let liv_path = format!("{}_{:?}_{:.2}.txt", base_config.liv_path, variant_attr, i);
                fname = format!("{}{:?}_{:.2}.yaml", target_dir, variant_attr, i);
            }
            Variants::off_f => {
                if i > 1.0 {
                    warn!("Offset_far > 1 -- did you include this in the test sequence pattern?")
                }
                if i < 0.0 {
                    warn!("Offset_far negative will lead to off_n + 1 -- did you include this in the test sequence pattern?")
                }
                let liv_path = format!("{}_{:?}_{:.2}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    off_f: i,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.2}.yaml", target_dir, variant_attr, i);
            }
            Variants::off_n => {
                if i > 1.0 {
                    warn!("Offset_near > 1 -- did you include this in the test sequence pattern?")
                }
                if i < 0.0 {
                    warn!("Offset_near negative will lead to off_f + 1 -- did you include this in the test sequence pattern?")
                }
                let liv_path = format!("{}_{:?}_{:.2}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    off_n: i,
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{:.2}.yaml", target_dir, variant_attr, i);
            }
            Variants::sim_no => {
                let liv_path = format!("{}_{:?}_{:.0}.txt", base_config.liv_path, variant_attr, i);
                config = rogona::init::ConfigVars {
                    liv_path,
                    ..base_config.clone()
                };
                fname = format!("{}{:?}_{}.yaml", target_dir, variant_attr, i);
            }
        }

        // parse to yaml file
        let yaml = serde_yaml::to_string(&config);
        match yaml {
            Ok(str) => {
                let mut test = File::create(&fname).expect("unable to create this config");
                write!(&mut test, "{}", str).expect("unable to write this config");
            }
            Err(_) => panic!("No yaml was serialized"),
        }
    }
}


/* Command line args
[1] "addl"
[2] target dir recon
[3] base configuration recon
[4] base config learn
[5] variant attribute
[6] start
[7] step
[8] stop
*/
fn add_learns() {
    info!("Starting to build variant learn configurations.");

    let args: Vec<String> = env::args().collect();
    // assign args
    let target_dir_recon = &args[2];
    let base_config_recon =
        fs::read_to_string(&args[3]).expect("unable to find/read base config recon");
    let base_config_recon: molcom_recon::threshold_impl::ConfigVars =
        serde_yaml::from_str(&base_config_recon).unwrap();
    let base_config_learn =
        fs::read_to_string(&args[4]).expect("unable to find/read base config learn");
    let base_config_learn: molcom_recon::threshold_impl::ConfigVarsLearn =
        serde_yaml::from_str(&base_config_learn).unwrap();
    let variant_attr = match Variants::from_str(&args[5]) {
        Ok(v) => v,
        Err(msg) => panic!("{}", msg),
    };
    let start: f64 = args[6].parse::<f64>().unwrap_or(0.0);
    let step: f64 = args[7].parse::<f64>().unwrap_or(f64::MAX);
    let end: f64 = args[8].parse::<f64>().unwrap_or(0.0);

    // [TODO]: number of decimals for each attr individually
    // for every variant make a reconstruction and learn configuration
    for i in StepRange(start, end, step) {
        let config_recon = molcom_recon::threshold_impl::ConfigVars {
            mode_path: format!(
                "{}_{:?}_{:.2}.yaml",
                base_config_recon.mode_path, variant_attr, i
            ),
            tx_far_out_path: format! {"{}_{:?}_{:.2}", base_config_recon.tx_far_out_path, variant_attr, i},
            tx_near_out_path: format! {"{}_{:?}_{:.2}", base_config_recon.tx_near_out_path, variant_attr, i},
            csv_path: format!("{}_{:?}_{:.2}", base_config_recon.csv_path, variant_attr, i),
            ..base_config_recon
        };
        let config_learn = molcom_recon::threshold_impl::ConfigVarsLearn {
            test_seq_path: base_config_learn.test_seq_path.clone(),
            apply_path: format!("{}_{:?}_{:.2}", base_config_learn.apply_path, variant_attr, i),
            ..base_config_learn
        };

        // convert into yaml files
        let yaml = serde_yaml::to_string(&config_recon);
        match yaml {
            Ok(str) => {
                let fname = format!("{}{:?}_{:.2}.yaml", target_dir_recon, variant_attr, i);
                let mut test = File::create(&fname).expect("unable to create this recon config");
                write!(&mut test, "{}", str).expect("unable to write this recon config");
            }
            Err(_) => panic!("No yaml was serialized"),
        }
        let yaml = serde_yaml::to_string(&config_learn);
        match yaml {
            Ok(str) => {
                let fname = config_recon.mode_path;
                let mut test = File::create(&fname).expect("unable to create this learn config");
                write!(&mut test, "{}", str).expect("unable to write this learn config");
            }
            Err(_) => panic!("No yaml was serialized"),
        }
    }
}

/* Command line args
[1] "adda"
[2] target dir recon
[3] base configuration recon
[4] true/false (learn config try? mode path append with 0 to 3 ?)
[5] variant attribute
[6] start
[7] step
[8] stop
*/
fn add_applies() {
    info!("Starting to build variant apply configurations.");

    let args: Vec<String> = env::args().collect();
    // assign args
    let target_dir_recon = &args[2];
    let base_config_recon =
        fs::read_to_string(&args[3]).expect("unable to find/read base config recon");
    let base_config_recon: molcom_recon::threshold_impl::ConfigVars =
        serde_yaml::from_str(&base_config_recon).unwrap();
    let append_try_cases = args[4].parse::<bool>().unwrap_or(false);
    let variant_attr = match Variants::from_str(&args[5]) {
        Ok(v) => v,
        Err(msg) => panic!("{}", msg),
    };
    let start: f64 = args[6].parse::<f64>().unwrap_or(0.0);
    let step: f64 = args[7].parse::<f64>().unwrap_or(f64::MAX);
    let end: f64 = args[8].parse::<f64>().unwrap_or(0.0);

    let mut j = 0;
    loop {
        // [TODO]: number of decimals for each attr individually
        // for every variant make a reconstruction (apply)
        for i in StepRange(start, end, step) {
            let config_recon = molcom_recon::threshold_impl::ConfigVars {
                mode_path: match append_try_cases {
                    true => format!(
                        "{}_{:?}_{:.2}_{}.yaml",
                        base_config_recon.mode_path, variant_attr, i, j
                    ),
                    false => format!(
                        "{}_{:?}_{:.2}_0.yaml", // TODO check whether the 0 can be left out
                        base_config_recon.mode_path, variant_attr, i
                    ),
                },
                tx_far_out_path: format! {"{}_{:?}_{:.2}", base_config_recon.tx_far_out_path, variant_attr, i},
                tx_near_out_path: format! {"{}_{:?}_{:.2}", base_config_recon.tx_near_out_path, variant_attr, i},
                csv_path: format!("{}_{:?}_{:.2}_{}.csv", base_config_recon.csv_path, variant_attr, i, j),
                ..base_config_recon
            };

            // convert into yaml files
            let yaml = serde_yaml::to_string(&config_recon);
            match yaml {
                Ok(str) => {
                    let fname = match append_try_cases {
                        true => format!("{}{:?}_{:.2}_{}.yaml", target_dir_recon, variant_attr, i, j),
                        false => format!("{}{:?}_{:.2}_0.yaml", target_dir_recon, variant_attr, i)};    // TODO check for leaving out 0
                    let mut test =
                        File::create(&fname).expect("unable to create this recon config");
                    write!(&mut test, "{}", str).expect("unable to write this recon config");
                }
                Err(_) => panic!("No yaml was serialized"),
            }
        }
        if append_try_cases == false || j >= 3 {
            break;
        } else {
            j += 1;
        }
    }
}