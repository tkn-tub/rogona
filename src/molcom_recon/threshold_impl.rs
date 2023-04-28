use log::{debug};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    env,
    fs::{self, File},
};


// With kind help from this library
use statrs::statistics::Distribution;
use statrs::statistics::Max;
use statrs::statistics::Median;
use statrs::statistics::Min;
use statrs::statistics::{Data, OrderStatistics};

use super::evaluation::EvaluationMethod;
use super::reconstruction::{self};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigVars {
    pub recon_mode: ReconstructionMode,
    pub mode_path: String, // Learn Mode: Out, Apply Mode: In
    pub tx_far_out_path: String,
    pub tx_near_out_path: String,
    pub csv_path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum ReconstructionMode {
    Learn,
    Apply,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigVarsLearn {
    pub th_cal: ThCalibration,
    pub test_seq_path: String,
    pub apply_path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigVarsApply {
    frame: usize,
    th: Threshold,
    th_cal: ThCalibration,
    order: ThOrder,
}

// Threshold Calculation Methods
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum ThCalibration {
    MinMax,
    Quartile,
    Median,
    ArithMean,
    Try,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum ThOrder {
    HFNL,
    HNFL,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
struct Threshold {
    large: f64,
    medium: f64,
    small: f64,
}

struct Reconstructor {
    msg_far: Vec<usize>,
    msg_near: Vec<usize>,
    binary_msg: bool,
}

impl Reconstructor {
    fn new() -> Reconstructor {
        Reconstructor {
            msg_far: vec![],
            msg_near: vec![],
            binary_msg: env::var("BINARY_MSG").is_ok(),
        }
    }

    // split the codes into bit
    fn sort(&mut self, value: f64, th: Threshold, order: ThOrder) {
        if value < th.small {
            self.msg_far.push(0);
            self.msg_near.push(0);
        } else if value < th.medium {
            if order == ThOrder::HNFL {
                self.msg_far.push(1);
                self.msg_near.push(0);
            } else {
                self.msg_far.push(0);
                self.msg_near.push(1);
            }
        } else if value < th.large {
            if order == ThOrder::HNFL {
                self.msg_far.push(0);
                self.msg_near.push(1);
            } else {
                self.msg_far.push(1);
                self.msg_near.push(0);
            }
        } else {
            self.msg_far.push(1);
            self.msg_near.push(1);
        }
    }

    fn print_msg(&mut self, path_tx_far: &str, path_tx_near: &str) {
        if self.binary_msg {
            reconstruction::write_binary(path_tx_far, self.msg_far.iter());
            reconstruction::write_binary(path_tx_near, self.msg_near.iter());
        } else {
            todo!();
            // converts to characters ?
        }
    }
}

// Threshold implementation of reconstruction with Learn or Apply Mode
pub fn reconstruct(sim_config_path: &str, recon_config_path: &str) -> EvaluationMethod {
    let s = fs::read_to_string(recon_config_path).expect("Reconstruction Configuration unreadable");
    let recon_config: ConfigVars = serde_yaml::from_str(&s).unwrap();

    let eval_method;
    match recon_config.recon_mode {
        ReconstructionMode::Learn => {
            let (path, cal) = learn(sim_config_path, &recon_config);
            eval_method = EvaluationMethod::Learn(path, cal);
            debug!("Learning sequence transmission time should take at least as long as the simulation time. -> Possibly significant errors in setting the threshold otherwise.");
            apply(sim_config_path, &recon_config, &eval_method);
        }
        ReconstructionMode::Apply => {
            let s = fs::read_to_string(&recon_config.mode_path)
                .expect("Apply Configuration not found/unreadable");
            let apply_config: ConfigVarsApply = serde_yaml::from_str(&s).unwrap();
            // add columns for apply csv
            if env::var("CSV_PRINT").is_ok() {
                let mut csv_file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&recon_config.csv_path)
                    .expect("csv file not found in apply.");
                csv_file
                    .write_all(b",,,,,,")
                    .expect("unable to write into csv file in apply");
            }
            eval_method = EvaluationMethod::Apply(apply_config.frame, apply_config.th_cal);
            apply(sim_config_path, &recon_config, &eval_method);
        }
    }
    eval_method
}


fn learn(sim_config_path: &str, recon_config: &ConfigVars) -> (String, ThCalibration) {
    let s = fs::read_to_string(&recon_config.mode_path)
        .expect("Learn Configuration unreadable/not found");
    let learn_config: ConfigVarsLearn = serde_yaml::from_str(&s).unwrap();

    // get frame, livs at frame and order (HFNL or HNFL) from reconstruction module
    let (frame, livs, order) = reconstruction::get_liv_synced(sim_config_path, None);

    let th: Vec<(ThCalibration, Threshold)> = calculate_thresholds(
        livs,
        &learn_config.test_seq_path,
        learn_config.th_cal,
        order,
    );

    // print Apply Configs
    let mut i = 0;
    for (cal, t) in th {
        let apply_config_yaml = serde_yaml::to_string(&ConfigVarsApply {
            frame,
            th: t,
            th_cal: cal,
            order: order,
        });
        match apply_config_yaml {
            Ok(str) => {
                let file_path = format!("{}_{}.yaml", &learn_config.apply_path, i);
                let mut apply_yaml =
                    File::create(&file_path).expect("creating apply file impossible");
                write!(&mut apply_yaml, "# These Thresholds have been created by the learn function of the reconstruction part in the Threshold Implementation\n{}", str).expect("unable to writte in apply file")
            }
            Err(msg) => panic!("{}", msg),
        }
        // csv for parameter studies
        if env::var("CSV_PRINT").is_ok() {
            let csv_path = format!("{}_{}.csv", &recon_config.csv_path, i);

            if env::var("APPEND").is_ok() {
                let mut csv_file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(csv_path)
                    .expect("csv file not found in learn.");
                let data = format!(
                    "{:?},{},{},{},{},{:?},",
                    cal, frame, t.large, t.medium, t.small, order
                );
                csv_file
                    .write_all(data.as_bytes())
                    .expect("unable to append to csv file in learn");
            } else {
                let data = format!("Th_cal,frame,B,M,S,Order,Bit_amount,error_count(near),BER_(near),0->1_(near),1->0_(near),error_count(far),BER_(far),0->1_(far),1->0_(far)\n{:?},{},{},{},{},{:?},",cal,frame,t.large,t.medium,t.small,order);
                let mut f = File::create(&csv_path).expect("unable to create csv-file");
                f.write_all(data.as_bytes())
                    .expect("unable to write csv data.");
            }
        }

        i += 1;
    }
    (learn_config.apply_path.to_string(), learn_config.th_cal)
}

fn apply(sim_config_path: &str, recon_config: &ConfigVars, eval_method: &EvaluationMethod) {
    match eval_method {
        EvaluationMethod::Apply(_, _) => {
            let s = fs::read_to_string(&recon_config.mode_path)
                .expect("Apply Configuration unreadable/not found");
            apply_subroutine(
                sim_config_path,
                &s,
                &recon_config.tx_far_out_path,
                &recon_config.tx_near_out_path,
            )
        }
        EvaluationMethod::Learn(path, cal) => {
            let variants = create_calibration_array(cal);
            let mut i = 0;
            for _ in variants {
                let mode_path = format! {"{}_{}.yaml", path, i};
                let s = fs::read_to_string(&mode_path)
                    .expect("Apply Configuration unreadable/not found");
                let path_tx_far = format!("{}_{}", &recon_config.tx_far_out_path, i);
                let path_tx_near = format!("{}_{}", &recon_config.tx_near_out_path, i);
                apply_subroutine(sim_config_path, &s, &path_tx_far, &path_tx_near);
                i += 1;
            }
        }
    };
}

// prints reconstructed message
fn apply_subroutine(
    sim_config_path: &str,
    apply_config_str: &str,
    path_tx_far: &str,
    path_tx_near: &str,
) {
    let apply_config: ConfigVarsApply = serde_yaml::from_str(apply_config_str).unwrap();
    let (_, livs, _) = reconstruction::get_liv_synced(sim_config_path, Some(apply_config.frame));

    let mut recon = Reconstructor::new();
    for val in livs {
        recon.sort(val, apply_config.th, apply_config.order);
    }
    println!("Order: {:?}", apply_config.order);
    recon.print_msg(path_tx_far, path_tx_near);
}

/* expects testsequence:
e.g.
f: 1100 1001 1010 1001 1100 1010
n: 1001 1100 1001 1010 1010 1100
-->
h,f,l,n,h,n,l,f,h,l,f,n,h,l,n,f,h,f,n,l,h,n,f,l

(f, n):
h: (1,1)
n: (0,1)
f: (1,0)
l: (0,0)

*/
fn calculate_thresholds(
    liv_deterministic: Vec<f64>,
    test_seq_path: &str,
    recon_version: ThCalibration,
    order: ThOrder,
) -> Vec<(ThCalibration, Threshold)> {
    let mut h = vec![];
    let mut n = vec![];
    let mut f = vec![];
    let mut l = vec![];

    let test_seq = fs::read_to_string(test_seq_path).expect("Could not read Testsequence Path");
    let test_seq = test_seq.split(',');
    let mut ts = vec![];
    for v in test_seq {
        ts.push(v.parse::<char>().unwrap_or('l').to_owned());
    }

    for (i, &v) in liv_deterministic.iter().enumerate() {
        match ts[i % ts.len()] {
            'h' => h.push(v),
            'n' => n.push(v),
            'f' => f.push(v),
            'l' => l.push(v),
            _ => panic!(
                "An invalid configuration in the testsequence was given. Please choose from 
            (f, n):
            h: (1,1)
            n: (0,1)
            f: (1,0)
            l: (0,0)
            according to your message."
            ),
        }
    }

    let mut th_vec: Vec<(ThCalibration, Threshold)> = vec![];

    let th_cals = create_calibration_array(&recon_version);

    let h = Data::new(h);
    let n = Data::new(n);
    let f = Data::new(f);
    let l = Data::new(l);

    for cal in th_cals {
        let th = if order == ThOrder::HNFL {
            Threshold {
                large: threshold_from(h.clone(), n.clone(), cal),
                medium: threshold_from(n.clone(), f.clone(), cal),
                small: threshold_from(f.clone(), l.clone(), cal),
            }
        } else {
            Threshold {
                large: threshold_from(h.clone(), f.clone(), cal),
                medium: threshold_from(f.clone(), n.clone(), cal),
                small: threshold_from(n.clone(), l.clone(), cal),
            }
        };
        th_vec.push((cal, th));
    }

    th_vec
}

pub fn create_calibration_array(version: &ThCalibration) -> Vec<ThCalibration> {
    let mut cals = vec![];

    match version {
        ThCalibration::Try => {
            cals.push(ThCalibration::MinMax);
            cals.push(ThCalibration::Quartile);
            cals.push(ThCalibration::Median);
            cals.push(ThCalibration::ArithMean);
        }
        calibration => cals.push(*calibration),
    }
    cals
}

fn threshold_from(
    mut high: Data<Vec<f64>>,
    mut low: Data<Vec<f64>>,
    version: ThCalibration,
) -> f64 {
    if high.is_empty() || low.is_empty() {
        panic!("Not enough data for threshold approach!");
    }
    let res;
    match version {
        ThCalibration::MinMax => {
            res = minmax(high, low);
        }

        ThCalibration::Quartile => res = (high.lower_quartile() + low.upper_quartile()) / 2.0, // ! NaN values unchecked

        ThCalibration::Median => res = (high.median() + low.median()) / 2.0, // ! NaN values unchecked

        ThCalibration::ArithMean => {
            debug!("Code panics here if high or low have a NaN value.");
            res = (high.mean().unwrap() + low.mean().unwrap()) / 2.0;
        }

        _ => panic!("Not a valid Threshold Calculation Calibration"),
    }
    res
}

fn minmax(high: Data<Vec<f64>>, low: Data<Vec<f64>>) -> f64 {
    let a = high.min();
    let b = low.max();
    if a.is_nan() || b.is_nan() {
        panic!("During min() and max() a NaN value appeared!");
    }
    (a + b) / 2.0
}
