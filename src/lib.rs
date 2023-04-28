pub mod rogona;
pub mod molcom_recon;
pub mod config_yaml;

use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Mode {
    AddVariants,
    AddLearns,
    AddApplies,
    OneFile
}


impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addv" => Ok(Mode::AddVariants),
            "addl" => Ok(Mode::AddLearns),
            "adda" => Ok(Mode::AddApplies),
            //"tryv" => Ok(Mode::ComputeVariants),
            _ => Ok(Mode::OneFile),
        }
    }
}