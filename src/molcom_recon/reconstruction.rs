use core::panic;
use itertools_num::linspace;
use std::io::Write;
use std::iter::zip;
use std::slice::Iter;
use std::{env, fs};

use super::threshold_impl::ThOrder;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use statrs::distribution::{Continuous, Normal};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConfigVars {
    mps: usize, // mol per spray
    d: f64,     // distance between sprayers in m
    dy: f64,    // offset of the receiver in m
    t_sim: f64, // simulation time limit in s
    t_sym: f64, // duration of one symbol (spray_time + pause_time) in s
    t_sp: f64,  // spray_time
    t_dts: f64, // delta time step (attribute to camera shutter time)
    off_f: f64,
    off_n: f64,
    cam_fps: usize,
    v_mean: f64,  // initial spray velocity
    v_sigma: f64, // velocity sigma
    liv_path: String,
    cam_z: f64,
    cam_height: f64,
    cam_w_proj: f64,
}

fn get_parameters(config_path: &str) -> ConfigVars {
    // Ignore first line and then get values in this order:
    match config_path {
        "" => panic!("Config File is missing!"),
        _ => {
            let c = fs::read_to_string(config_path).expect("unable to find config_path in init");
            serde_yaml::from_str(&c).unwrap()
        }
    }
}

/**
 * Expects a file of numbers with ';' as splitsymbol
 * Parses them into f64 in a Vec<f64>
 * If it is nan, it adds 0.0
 */
pub fn get_values_from_file(file_path: &str) -> Vec<f64> {
    let contents = fs::read_to_string(file_path).expect("Value file unreadable");
    let mut v_arr: Vec<f64> = vec![];

    let split = contents.split(';');
    for v in split.into_iter() {
        v_arr.push(v.parse::<f64>().unwrap_or(0.0).to_owned());
    }

    v_arr
}

fn get_sync_frame_and_order(config: ConfigVars, tpf: f64) -> (usize, ThOrder) {
    let htx = config.cam_height - config.cam_z;
    let hrx = config.cam_height;
    let cam_width_tx = (1.0f64 - (htx / hrx)) * config.cam_w_proj;
    let (off_n, off_f) = correct_negative_offsets(config.off_n, config.off_f);

    // ! see thesis
    let dist_adj =
        -(cam_width_tx / 8.0 + 9.0 / 5120.0 * config.d / cam_width_tx);
    //let dist_adj = 0.0;
        
    // !

    let d_travel_near = (config.d / 2.0) - config.dy + dist_adj;
    let d_travel_far = (config.d / 2.0) + config.dy + dist_adj;

    let data_points = 5000;
    let vel: Vec<f64> = linspace::<f64>(
        config.v_mean - 3.0 * config.v_sigma,
        config.v_mean + 3.0 * config.v_sigma,
        data_points,
    )
    .collect(); // 3 Sigma distance for 99.7% of all molecules
    let v_nd =
        Normal::new(config.v_mean, config.v_sigma).expect("Could not make a normal distribution");
    let p_nd: Vec<f64> = vel.iter().map(|&v| v_nd.pdf(v)).collect();

    // * STUDY 5
    /* let mut debug_python = fs::File::create("study_results/study5/Calc_diagrams/vel.txt").expect("unable to create vel.txt");
    for val in vel.clone() {
        write!(&mut debug_python, "{};", val).expect("unable to write into vel.txt");
    }

    let mut debug_python = fs::File::create("study_results/study5/Calc_diagrams/normal_dist_rust.txt").expect("unable to create normal_dist_rust.txt");
    for val in p_nd.clone() {
        write!(&mut debug_python, "{};", val).expect("unable to write into normal_dist_rust.txt");
    } */
    // * STUDY 5

    // this has an equal amount of values as p_nd and every interval has the according weighting ("space dimension")
    let t_travel_near_interval: Vec<(f64, f64)> = vel
        .iter()
        .map(|&v| {
            (
                (d_travel_near - cam_width_tx / 2.0) / v + (off_n % 1.0 * config.t_sym),
                (d_travel_near + cam_width_tx / 2.0) / v + (off_n % 1.0 * config.t_sym),
            )
        })
        .collect();
    let t_travel_far_interval: Vec<(f64, f64)> = vel
        .iter()
        .map(|&v| {
            (
                (d_travel_far - cam_width_tx / 2.0) / v + (off_f % 1.0 * config.t_sym),
                (d_travel_far + cam_width_tx / 2.0) / v + (off_f % 1.0 * config.t_sym),
            )
        })
        .collect();



    let mean_time_near = d_travel_near / config.v_mean + off_n % 1.0 * config.t_sym;
    let mean_time_far = d_travel_far / config.v_mean + off_f % 1.0 * config.t_sym;
    debug!("MTN: {:.3}; MTF: {:.3}", mean_time_near, mean_time_far);

    // experimentally good values
    let v_isi_bound_1st_half = config.v_mean - 0.5 * config.v_sigma;
    let v_isi_bound_2nd_half = config.v_mean + 2.0 * config.v_sigma;
    debug!(
        "v1st: {:.3}; v2nd: {:.3}",
        v_isi_bound_1st_half, v_isi_bound_2nd_half
    );

    let isi_bound_1st_time_near = d_travel_near / v_isi_bound_1st_half + off_n % 1.0 * config.t_sym;
    let isi_bound_1st_time_far = d_travel_far / v_isi_bound_1st_half + off_f % 1.0 * config.t_sym;
    debug!(
        "t1stn: {:.3}; t1stf: {:.3}",
        isi_bound_1st_time_near, isi_bound_1st_time_far
    );

    let isi_bound_2nd_time_near = d_travel_near / v_isi_bound_2nd_half + off_n % 1.0 * config.t_sym;
    let isi_bound_2nd_time_far = d_travel_far / v_isi_bound_2nd_half + off_f % 1.0 * config.t_sym;
    debug!(
        "t2ndn: {:.3}; t2ndf: {:.3}",
        isi_bound_2nd_time_near, isi_bound_2nd_time_far
    );

    // ftr - first-to-reach; ltr - last-to-reach
    let (isi_bound_1st_time_ftr, isi_bound_2nd_time_ftr) = if mean_time_near < mean_time_far {
        (isi_bound_1st_time_near, isi_bound_2nd_time_near)
    } else {
        (isi_bound_1st_time_far, isi_bound_2nd_time_far)
    };

    debug!(
        "FTR: {}",
        if isi_bound_1st_time_ftr == isi_bound_1st_time_far {
            "FAR"
        } else {
            "NEAR"
        }
    );

    // possible frames for reconstruction

    let min_frame;
    let max_frame;

    // * for max printout
    if env::var("RECON_FULL_PRINT").is_ok() {
        min_frame = 0;
        max_frame = ((d_travel_far / (config.v_mean - 3.0 * config.v_sigma) + config.t_sp) / tpf)
            .ceil() as usize;
    } else {
        min_frame = ((isi_bound_1st_time_ftr + config.t_sp) / tpf).ceil() as usize; // * avoid first half ISI and N steepness
        max_frame = ((isi_bound_2nd_time_ftr + config.t_sym) / tpf).ceil() as usize;
        // * avoid second half ISI
    }

    if max_frame < min_frame {
        return (
            (max_frame + min_frame) / 2,
            if mean_time_near < mean_time_far {
                ThOrder::HNFL
            } else {
                ThOrder::HFNL
            },
        );
    }

    let relevant_frames = max_frame - min_frame + 1;

    if max_frame < min_frame {
        warn!("Very high ISI effects expected. Frame Calculation does not return a result.\nReduce Offsets and retry.");
        return (
            (max_frame + min_frame) / 2,
            if mean_time_near < mean_time_far {
                ThOrder::HNFL
            } else {
                ThOrder::HFNL
            },
        );
    }

    println!("Min Frame: {}\nMax Frame: {}\n", min_frame, max_frame);
    let mut p_sum_n: Vec<(f64, usize)> = vec![(0.0, 0); relevant_frames];
    let mut p_sum_f: Vec<(f64, usize)> = vec![(0.0, 0); relevant_frames];

    // for all molecules slightly offset during spray duration ("time dimension")
    for i in 0..config.mps {
        let t_n: Vec<(f64, f64)> = t_travel_near_interval
            .iter()
            .map(|&(t_min, t_max)| {
                (
                    t_min + config.t_sp * (i as f64 / config.mps as f64),
                    t_max + config.t_sp * (i as f64 / config.mps as f64),
                )
            })
            .collect();
        let t_f: Vec<(f64, f64)> = t_travel_far_interval
            .iter()
            .map(|&(t_min, t_max)| {
                (
                    t_min + config.t_sp * (i as f64 / config.mps as f64),
                    t_max + config.t_sp * (i as f64 / config.mps as f64),
                )
            })
            .collect();
        let (_, p_n) = frame_prob_calc(t_n, p_nd.clone(), tpf, max_frame, min_frame);
        let (_, p_f) = frame_prob_calc(t_f, p_nd.clone(), tpf, max_frame, min_frame);

        p_sum_n = zip(p_sum_n, p_n)
            .map(|((p_sum, f), p_nd)| (p_sum + p_nd, f + 1))
            .collect();
        p_sum_f = zip(p_sum_f, p_f)
            .map(|((p_sum, f), p_nd)| (p_sum + p_nd, f + 1))
            .collect();
    }

    // norm from adding up over time and multiply with respective enhancing/damping factor depending on dy and d
    let p_sum_n_normal: Vec<f64> = p_sum_n
        .into_iter()
        .map(|(v, f)| (v / f as f64) * (1.0 + config.dy + 0.5 * config.dy / config.d))
        .collect();
    let p_sum_f_normal: Vec<f64> = p_sum_f
        .into_iter()
        .map(|(v, f)| (v / f as f64) * (1.0 - config.dy - 0.05 * config.dy / config.d))
        .collect();

    // printing for visualization
    if env::var("RECON_PRINT").is_ok() {
        let mut debug_python = fs::File::create(format!("{}_dist_near.txt", &(env::args().collect::<Vec<String>>())[5]))
        .expect("unable to create dist_near.txt");
        for val in p_sum_n_normal.clone() {
            write!(&mut debug_python, "{};", val).expect("unable to write into dist_near.txt");
        }

        let mut debug_python = fs::File::create(
            format!("{}_dist_far.txt", &(env::args().collect::<Vec<String>>())[5])
        )
        .expect("unable to create dist_far.txt");
        for val in p_sum_f_normal.clone() {
            write!(&mut debug_python, "{};", val).expect("unable to write into dist_far.txt");
        }
    }

    let diff_and_h_diff_signed: Vec<(f64, f64)> = zip(p_sum_n_normal, p_sum_f_normal)
        .map(|(n, f)| (n - f, if n < f { n } else { f }))
        .collect();
    let sign: Vec<bool> = diff_and_h_diff_signed
        .iter()
        .map(|&(i, _)| i > 0.0)
        .collect();

    let diff_and_h_diff_unsigned: Vec<(f64, f64)> = diff_and_h_diff_signed.iter().map(|&(fndiff, hdiff)| (fndiff.abs(), hdiff.abs())).collect();

    if env::var("RECON_PRINT").is_ok() {
        let mut diff_fn: Vec<f64> = vec![];
        let mut diff_h: Vec<f64> = vec![];
        for (dfn, dh) in diff_and_h_diff_unsigned.clone().into_iter() {
            diff_fn.push(dfn);
            diff_h.push(dh);
        }

        let mut debug_python = fs::File::create(format!("{}_diff_fn.txt", &(env::args().collect::<Vec<String>>())[5]))
        .expect("unable to create diff_fn.txt");
        for val in diff_fn.clone() {
            write!(&mut debug_python, "{};", val).expect("unable to write into diff_fn.txt");
        }
        let mut debug_python = fs::File::create(format!("{}_diff_h.txt", &(env::args().collect::<Vec<String>>())[5]))
        .expect("unable to create diff_h.txt");
        for val in diff_h.clone() {
            write!(&mut debug_python, "{};", val).expect("unable to write into diff_h.txt");
        }
    }


    // Choose a frame with skewing.
    let max_diff_index = double_max(diff_and_h_diff_unsigned, sign.clone());
    

    //let max_diff_index = get_max_index(&diff);
    let order;
    if sign[max_diff_index] == true {
        order = ThOrder::HNFL;
    } else {
        order = ThOrder::HFNL;
    }

    let max_diff_frame = match env::var("FRAME_OFFSET") {
        Ok(off) => (max_diff_index + min_frame) as i32 + off.parse::<i32>().expect("FRAME_OFFSET (env var) must be an int"),
        Err(_) => (max_diff_index + min_frame) as i32
    };

    println!("Max Diff Frame: {}", max_diff_frame);
    (max_diff_frame.try_into().expect("This is a negative frame!"), order)
    
}

// reduces all LIVs to the one at reconstruction frame
pub fn get_liv_synced(config_path: &str, frame: Option<usize>) -> (usize, Vec<f64>, ThOrder) {
    let config = get_parameters(config_path);

    let livs = get_values_from_file(&config.liv_path);

    let tpf = match env::var("SIM2VID").is_ok() {
        true => 1.0 / (config.cam_fps as f64),
        false => config.t_dts,
    }; // "time per frame"
    let fpsym = (config.t_sym / tpf).round() as usize; // "frames per symbol"

    let (frame, order) = match frame {
        Some(f) => (f, ThOrder::HFNL), // ! if frame is given, order must be given too -> disregards return parameter order
        None => get_sync_frame_and_order(config, tpf),
    };

    let mut livs_synced = vec![];
    let n_bit = (livs.len() - 1 - frame) / fpsym;
    for i in 0..n_bit {
        livs_synced.push(livs[i * fpsym + frame])
    }

    (frame, livs_synced, order)
}

fn get_max_index<T: PartialOrd + Copy>(arr: &[T]) -> usize {
    let mut max = &arr[0];
    let mut max_index = 0;

    for (index, val) in arr.iter().enumerate() {
        if val > max {
            max = val;
            max_index = index;
        }
    }

    max_index
}

pub fn arith_mean(arr: Vec<f64>) -> f64 {
    let a: f64 = arr.iter().sum();
    let b = arr.len();
    match b {
        0 => 0.0,
        _ => a / (b as f64),
    }
}

/**
 * t_arr: Array with travel times // ! ordered from longest to shortest -- gives runtime and spce improvement
 * p_t_arr: Array with probabilities of the respective travel times
 * tpf: time per frame
 * off: additional frames to look at
 * 
 * returns a probability of the molecule appearing in the frame
 */
fn frame_prob_calc(
    t_arr: Vec<(f64, f64)>,
    p_t_arr: Vec<f64>,
    tpf: f64,
    max_fr: usize,
    min_fr: usize,
) -> (Vec<usize>, Vec<f64>) {
    let mut frame = Vec::with_capacity(max_fr - min_fr + 1);
    let mut probability = Vec::with_capacity(max_fr - min_fr + 1);

    for i in min_fr..max_fr + 1 {
        let mut arr = vec![];
        let mut j = 0;
        for &(t_min, t_max) in &t_arr {
            if (i as f64) * tpf <= t_max && (i as f64) * tpf >= t_min {
                arr.push(p_t_arr[j]);
            }
            j += 1;
        }

        let p_v = arith_mean(arr);
        frame.push(i);
        probability.push(p_v);
    }

    (frame, probability)
}

// Relative Offset to receiver and other injector is consistent. The exact frame offset cannot be assured if t_sym % t_dts or camera fps != 0.
pub fn correct_negative_offsets(mut off1: f64, mut off2: f64) -> (f64, f64) {
    while off1 < 0.0 || off2 < 0.0 {
        off1 += 1.0;
        off2 += 1.0;
    }

    (off1, off2)
}

pub fn delete_offset(values: &mut Vec<f64>, off: usize) -> &mut Vec<f64> {
    let mut off_count = off;
    while off_count > 0 {
        values.pop();
        off_count -= 1;
    }
    values
}

pub fn write_binary(path: &str, values: Iter<usize>) {
    let mut file = fs::File::create(path).unwrap(); // create the file if it doesn't exist. Empty it if it does
    for &v in values {
        if v == 0 || v == 1 {
            write!(&mut file, "{}", v).expect("writing in the file not possible");
        } else {
            panic!("Something went wrong in pushing the bits while sorting");
        }
    }
}

// chooses frame. skewed.
pub fn double_max(unsigned_arrays: Vec<(f64, f64)>, signs: Vec<bool>) -> usize {
    let mut arr1: Vec<f64> = vec![];
    let mut arr2: Vec<f64> = vec![];
    for (x,y) in unsigned_arrays.into_iter() {
        arr1.push(x);
        arr2.push(y);
    }

    let mut max_index_1: usize = get_max_index(&arr1);
    let mut max_index_2: usize = get_max_index(&arr2);
    if arr1[max_index_1] < 0.70*arr2[max_index_2]{
        return max_index_1;
    }
    if arr1[max_index_2] < 0.25*arr2[max_index_2] {
        arr2[max_index_2] = 0.0;
        max_index_2 = get_max_index(&arr2);
    }
    if arr2[max_index_1] < 0.2*arr1[max_index_1] {
        arr1[max_index_1] = 0.0;
        max_index_1 = get_max_index(&arr1);
    }

    let mut max_index_1_prev = max_index_1;
    let mut max_index_2_prev = max_index_2;
    
    loop{
        if max_index_1 == max_index_2 {
            if signs[max_index_1] { // HNFL
                return max_index_2_prev;
            } else {
                return max_index_1_prev;
            }
        } else {
            if arr1.len()%2 == 0 || arr2.len()%2 == 0 {
                if (max_index_1-max_index_2) == 1 {
                    if signs[max_index_1] { // HNFL
                        return max_index_2_prev;
                    } else {
                        return max_index_1_prev;
                    }
                }
            }
            arr1[max_index_1] = 0.0;
            arr2[max_index_2] = 0.0;
            max_index_1_prev = max_index_1;
            max_index_2_prev = max_index_2;
        }

        max_index_1 = get_max_index(&arr1);
        max_index_2 = get_max_index(&arr2);
    }
}




























/* KMeans Impl */

/*
   Compare init_kmeanplusplus (0); init_random_partition (1); init_random_sample (2)
*/

/* mod k_means_reconstruction {
    // https://rust-ml.github.io/book/3_kmeans.html
    use linfa::prelude::*;
    use linfa_clustering::KMeans;

    use ndarray::prelude::*;
    use rand::prelude::*;

    use plotters::prelude::*;


} */




#[cfg(test)]
mod test {
    use std::iter::zip;

    use super::*;

    /* #[test]
    fn read_config(){
        let fn_config = get_parameters("yaml_configs/form.txt");
        let hard_config = ConfigVars{mps: 1000, d: 1.18, dy:0.03, t_sim: 4.0, t_sym: 0.05, t_sp: 0.02, t_dts: 0.002, cam_fps: 480, v_mean: 12.82, v_sigma: 3.0, liv_path: "livs.txt".to_string()};
        assert_eq!(fn_config, hard_config)
    } */

    #[test]
    fn max_index() {
        let arr = vec![0.0, 2.0, 5.0, 3.0];
        let max_index = get_max_index(&arr);
        assert_eq!(max_index, 2);
    }

    #[test]
    fn zip_test() {
        let a = vec![0, 2];
        let b = vec![1];
        let mut c = zip(a, b);
        assert_eq!(c.next().unwrap(), (0, 1));
        assert!(c.next().is_none());
    }

    #[test]
    fn arithmean() {
        let arr = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(arith_mean(arr), 2.5)
    }

    #[test]
    fn frameprob() {
        let t_arr = vec![(1.0, 3.0), (2.0, 4.0)];
        let p_t_arr = vec![0.5, 0.8];
        let tpf = 0.5;
        let max_fr = 9;
        let min_fr = 0;

        println!(
            "{:#?}",
            frame_prob_calc(t_arr, p_t_arr, tpf, max_fr, min_fr)
        )
    }

    #[test]
    fn fmod() {
        let i = 1.25;
        assert_eq!(0.25, i % 1.0);
    }
}
