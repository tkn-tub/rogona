use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{env, fs};

use super::{
    reconstruction::correct_negative_offsets,
    threshold_impl::{create_calibration_array, ThCalibration},
};

pub enum EvaluationMethod {
    Learn(String, ThCalibration),
    Apply(usize, ThCalibration),
}

enum BitFlip {
    None,
    ToOne,
    ToZero,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct InConfig {
    off_f: f64,
    msg_f: String,
    rep_f: usize,
    off_n: f64,
    msg_n: String,
    rep_n: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OutConfig {
    tx_far_out_path: String,
    tx_near_out_path: String,
    csv_path: String,
}


/* CSV format:
Th_cal , frame , B , M , S , Order , Bit amount , error count(near), BER (near), 0->1 (near), 1->0 (near) , Bit amount, error count(far), BER (far), 0->1 (far), 1->0 (far)
*/
#[derive(Debug)]
pub struct CsvVector {
    pub bit: usize,
    pub error: usize,
    pub ber: f64,
    pub toone: usize,
    pub tozero: usize,
}

pub fn evaluate(sim_config_path: &str, recon_config_path: &str, method: EvaluationMethod) {
    let a =
        fs::read_to_string(sim_config_path).expect("Simulation Configuration unreadable/not found");
    let in_config: InConfig = serde_yaml::from_str(&a).unwrap();
    let b = fs::read_to_string(recon_config_path)
        .expect("Reconstruction Configuration unreadable/not found");
    let out_config: OutConfig = serde_yaml::from_str(&b).unwrap();
    let (off_n, off_f) = correct_negative_offsets(in_config.off_n, in_config.off_f);

    let binary_msg = env::var("BINARY_MSG").is_ok();

    match method {
        EvaluationMethod::Apply(frame, cal) => {

            let mut csv_file = match fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&out_config.csv_path)
                {
                    Ok(file) => file,
                    Err(msg) => {
                        if env::var("CSV_PRINT").is_ok() {
                            panic!("Did not find csv file in this path; {}", msg)
                        } else {
                            fs::File::create("dummy.csv").expect("unable to create dummy file")
                        }
                    }
                };
            println!(
                "_____________________________\nUsing {:?} at frame {}\n",
                cal, frame
            );

            // evaluate msg from near sprayer
            let cv = bit_error(
                &in_config.msg_n,
                in_config.rep_n,
                off_n,
                &out_config.tx_near_out_path,
                binary_msg,
            );
            if env::var("CSV_PRINT").is_ok() {
                csv_file
                    .write_all(
                        format!(
                            "{},{},{},{},{},",
                            cv.bit, cv.error, cv.ber, cv.toone, cv.tozero
                        )
                        .as_bytes(),
                    )
                    .expect("unable to write into csv in eval");
            }

            // evaluate msg from far sprayer
            let cv = bit_error(
                &in_config.msg_f,
                in_config.rep_f,
                off_f,
                &out_config.tx_far_out_path,
                binary_msg,
            );
            if env::var("CSV_PRINT").is_ok() {
                csv_file
                    .write_all(
                        format!(
                            "{},{},{},{},{}\n",
                            cv.bit, cv.error, cv.ber, cv.toone, cv.tozero
                        )
                        .as_bytes(),
                    )
                    .expect("unable to write into csv in eval");
            }
        }
        EvaluationMethod::Learn(_, cal) => {
            let variants = create_calibration_array(&cal); // returns one (MinMax, Quartile, Median, Mean) or four configs (Try)
            let mut i = 0;
            for v in variants {
                let csv_path = format!("{}_{}.csv", out_config.csv_path, i);
                let mut csv_file = match fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&csv_path)
                {
                    Ok(file) => file,
                    Err(msg) => {
                        if env::var("CSV_PRINT").is_ok() {
                            panic!("Did not find csv file in this path; {}", msg)
                        } else {
                            fs::File::create("dummy.csv").expect("unable to create dummy file")
                        }
                    }
                };
                println!("_____________________________\nUsing {:?}\n", v);

                // evaluate msg from near sprayer
                let cv = bit_error(
                    &in_config.msg_n,
                    in_config.rep_n,
                    off_n,
                    &format!("{}_{}", &out_config.tx_near_out_path, i),
                    binary_msg,
                );
                if env::var("CSV_PRINT").is_ok() {
                    csv_file
                        .write_all(
                            format!(
                                "{},{},{},{},{},",
                                cv.bit, cv.error, cv.ber, cv.toone, cv.tozero
                            )
                            .as_bytes(),
                        )
                        .expect("unable to write into csv in eval");
                }
                //evaluate msg from far sprayer
                let cv = bit_error(
                    &in_config.msg_f,
                    in_config.rep_f,
                    off_f,
                    &format!("{}_{}", &out_config.tx_far_out_path, i),
                    binary_msg,
                );
                if env::var("CSV_PRINT").is_ok() {
                    csv_file
                        .write_all(
                            format!(
                                "{},{},{},{},{}\n",
                                cv.bit, cv.error, cv.ber, cv.toone, cv.tozero
                            )
                            .as_bytes(),
                        )
                        .expect("unable to write into csv in eval");
                }
                i += 1; // for Try file names 0 to 3
            }
        }
    }
}

fn bit_error(
    input_file_path: &str,
    input_repetitions: usize,
    input_offset: f64,
    output_file_path: &str,
    binary_msg: bool,
) -> CsvVector {
    // open files
    println!("Input: {}\nOutput: {}\n", input_file_path, output_file_path);

    let mut input = fs::read_to_string(input_file_path).expect("Cannot open input file");
    let output = fs::read_to_string(output_file_path).expect("Cannot open output file");

    // TODO[Future Work] convert output to binary to compare
    if binary_msg == false {}

    // account for delayed start time
    for _ in 0..(input_offset.floor() as i32) {
        input.pop();
    }

    // account for repeated messages
    let mut netto_input = String::new();
    for _ in 0..input_repetitions {
        netto_input = netto_input + &input;
    }

    // compare whether we have a bitflip

    let res = compare(netto_input, output);

    print_results(res)
}

fn compare(input: String, output: String) -> Vec<(bool, BitFlip)> {
    let mut input = input.chars();
    let mut output = output.chars();

    let mut result: Vec<(bool, BitFlip)> = vec![];

    loop {
        match (input.next(), output.next()) {
            (Some('0'), Some('0')) => result.push((true, BitFlip::None)),
            (Some('1'), Some('1')) => result.push((true, BitFlip::None)),
            (Some('0'), Some('1')) => result.push((false, BitFlip::ToOne)),
            (Some('1'), Some('0')) => result.push((false, BitFlip::ToZero)),
            (Some(_), Some(_)) => eprintln!("This was not converted to a binary message"),
            (Some(_), None) => break, //eprintln!("The message wasn't fully transmitted"),
            (None, Some('1')) => {
                log::debug!("Possibly ISI");
                result.push((false, BitFlip::ToOne))
            }
            (None, Some('0')) => break,
            (None, Some(_)) => eprintln!("This was not converted to a binary message"),
            (None, None) => break,
        }
    }

    result
}

fn print_results(mut res: Vec<(bool, BitFlip)>) -> CsvVector {
    let bit_count = res.len();
    println!("In this transmission of {} Bit", bit_count);

    let mut error_absolute: Vec<(bool, BitFlip)> =
        res.drain(..).filter(|(v, _)| *v == false).collect();
    let error = error_absolute.len();
    println!("{} errors were detected.\nResulting in a BER of {}", error, (error as f64)/(bit_count as f64));

    let mut to_one_err = 0;
    let mut to_zero_err = 0;

    for (_, bf) in error_absolute.drain(..) {
        match bf {
            BitFlip::ToOne => to_one_err += 1,
            BitFlip::ToZero => to_zero_err += 1,
            BitFlip::None => eprintln!("Something went wrong here before (counting the bitflips)"),
        }
    }

    println!(
        "Those were {} '0 -> 1' errors and {} '1 -> 0' errors\n",
        to_one_err, to_zero_err
    );

    CsvVector {
        bit: bit_count,
        error,
        ber: (error as f64) / (bit_count as f64),
        toone: to_one_err,
        tozero: to_zero_err,
    }
}
