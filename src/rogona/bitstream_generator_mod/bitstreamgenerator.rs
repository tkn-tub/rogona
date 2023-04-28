// TODO: [Future Work] File locks

/**This module reads a message and converts it to a bitsequence.
 * AS OF NOW:
 *  - It reads a whole file and transmits all chars a number of repititions.
 *  - Afterwards \0 is transmitted endlessly
 *
 * ! set env-variable BINARY_MSG = true if your file contains a bitsequence
*/
extern crate log;
use log::{debug, error, info, warn};

extern crate queues;
use queues::*;

use crate::rogona::{
    modulation_mod::modulation_ook::ModulationOOK,
    rg_attributes::pgtraits::PGObj
};

use std::env;
use std::fs;
use std::time;

#[derive(Debug)]
pub struct FileAttr {
    pub name: Option<String>,
    pub eof: bool,
}

#[derive(Debug)]
//pub struct Bitstreamgenerator<T: modulation::Modulation> {
pub struct Bitstreamgenerator {
    id: usize,
    start_time: time::Duration, // At what simulated time in seconds to start the sequence transmission.
    repetitions: u32,           // how often does the message get repeated
    file: FileAttr,             // where the sender inputs the message
    end_of_transmission: bool,
    ascii_array: Queue<char>, //Vec<char>, // convert into chars
    bit_sequence: u8,         // char as bit sequence for transmission
    sequence_count: u8,       // when new char is loaded
    binary_msg: bool,
}

impl PGObj for Bitstreamgenerator {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Bitstreamgenerator {
    pub fn new(
        id: usize,
        start_time: time::Duration,
        repetitions: u32,
        file_path: &str,
    ) -> Bitstreamgenerator {
        let file = FileAttr{name: Some(String::from(file_path)), eof: false};

        Bitstreamgenerator {
            id,
            start_time,
            repetitions,
            file,
            end_of_transmission: false,
            ascii_array: queue![],
            bit_sequence: 0,
            sequence_count: 10,
            binary_msg: env::var("BINARY_MSG").is_ok(),
        }
    }

    // returns a '1' or '0' (like a shift register)
    pub fn get_bit(&mut self, sim_time: &time::Duration) -> u8 {
        if self.end_of_transmission == true {
            return 0;
        }

        // get new char
        let mut val;
        if self.sequence_count >= 8 {
            
            self.sequence_count = 0;
            //loop in case ascii array is empty;
            loop {
                
                self.bit_sequence = match self.ascii_array.remove() {
                    Ok(v) => v as u8,
                    Err(_e) => {
                        val = self.fill_ascii_array();
                        match val {
                            Ok(_) => continue,
                            Err(e) => {
                                info!["no message to transmit: {} in {}", e, self.file.name.as_ref().unwrap()];
                                // TODO [Future Work] lift "TX ongoing lock" for SimStage::READ to read in a new file
                                self.end_of_transmission = true;
                                debug!["Last Bit's transmission at approx. {}", sim_time.as_secs_f64()];
                                warn!("Learning sequence transmission time should take at least as long as the simulation time! -- If this is an 'Apply' simulation, ignore this warning.");
                                '0' as u8
                            }
                        }
                    }
                };
                break;
            }
        }
        self.sequence_count += 1;

        if self.binary_msg == true {
            self.sequence_count = 8;
            if self.bit_sequence == '0' as u8 {
                return 0;
            } else if self.bit_sequence == '1' as u8 {
                return 1;
            } else {
                error!("The message is not typed as binary code! Please change environment variable!");
                println!("char = {}", self.bit_sequence);
                panic!("check ENV-Variable \"BINARY_MSG\"");
            }
        } else {
            let bit = self.bit_sequence >> 7;
            self.bit_sequence = self.bit_sequence << 1;
            match bit {
                1 => return 1,
                0 => return 0,
                _ => {
                    error!("unexpected value!");
                    return 0;
                }
            }
        }
    }


    fn fill_ascii_array(&mut self) -> Result<u8, &'static str> {
        if self.end_of_transmission == false && self.file.eof == false {
            let res = self.read_from_file();
            match res {
                Err(true) => {
                    info!("EOF");
                    self.file.eof = true; // ! EOF LOCK (For future work stage READ)
                    Err("EOF reached.")
                }
                Err(false) => {
                    error!("reading the file was not possible");
                    self.end_of_transmission = true; // ! locks simulation because file cannot be accessed
                    Err("cannot access file")
                }
                Ok(str) => {
                    let mut q = queue![];
                    for ch in str.chars() {
                        q.add(ch).unwrap(); // should be successful anyways
                    }
                    self.ascii_array = q;
                    Ok(0)
                }
            }
        } else {
            Err("EOF reached.")
        }
    }

    // read whole file and return as String //bool for EOF reached or file not readable!
    fn read_from_file(&mut self) -> Result<String, bool> {
        if self.file.eof == true {
            return Err(true);
        }
        self.repetitions -= 1;
        match &self.file.name {
            None => {
                warn!("No file specified.");
                if self.repetitions <= 0 {
                    self.file.eof = true;
                }
                Ok(String::from("Automatic message."))
            }
            Some(fname) => {
                debug!("Reading file {}", fname);
                if self.repetitions <= 0 {
                    self.file.eof = true;
                }
                match fs::read_to_string(fname) {
                    Ok(content) => Ok(content),
                    Err(_) => Err(false),
                }
            }
        }
    }

    fn read_sim(&mut self) {
        self.file.eof = false;
    }

    // Sets the Modulation up.
    pub fn bitsequence(&mut self, sim_time: &time::Duration, m: &mut Box<ModulationOOK>) {

        if *sim_time >= self.start_time {

            if m.transmission_locked() == false {
                match m.get_next_bit() {
                    Some(b) => m.set_current_bit(b),
                    None => {m.set_current_bit(self.get_bit(sim_time))},
                }
                m.set_next_bit(self.get_bit(sim_time));
                m.increase_symbol_count();
                m.lock_transmission();
                
               
            }
        }

    }
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}





#[cfg(test)]
mod tests {
    use core::panic;
    use std::fs::File;

    use super::*;

    fn use_bitsequence(version: u8) -> u8 {
        match version {
            0 => 0b10000010,
            1 => 'h' as u8,
            _ => 0,
        }
    }

    fn use_ascii_array(version: u8) -> Queue<char> {
        match version {
            1 => queue!['e', 'l', 'l', 'o', '!'],
            _ => queue![],
        }
    }

    fn use_ascii_string(version: u8) -> String {
        match version {
            2 => String::from("ello!"),
            3 => String::from("My name is!"),
            _ => String::new(),
        }
    }

    fn use_file(version: u8) -> FileAttr {
        match version {
            4 => FileAttr {
                name: Some(String::from("example.txt")),
                eof: false,
            },
            _ => FileAttr {
                name: None,
                eof: false,
            },
        }
    }

    #[test]
    // Test that the Bits are transmitted in the correct order
    fn get_bit_test() {
        let mut bg = Bitstreamgenerator {
            id: 1,
            start_time: time::Duration::new(0, 0),
            repetitions: 1,
            file: FileAttr {
                name: None,
                eof: true,
            },
            end_of_transmission: false,
            ascii_array: queue![],
            bit_sequence: use_bitsequence(0),
            sequence_count: 0,
            binary_msg: env::var("BINARY_MSG").is_ok(),
        };
        let mut seq = String::new();
        for _ in 0..8 {
            match bg.get_bit(&time::Duration::from_secs(0)) {
                1 => {
                    seq.push('1');
                }
                0 => {
                    seq.push('0');
                }
                _ => {
                    panic!("this shouldn't be possible")
                }
            }
        }
        assert_eq!(String::from("10000010"), seq); //uses version 0
    }

    #[test]
    //test whether a word from ascii_queue can be transmitted correctly
    fn get_bit_ascii_test() {
        let mut bg = Bitstreamgenerator {
            id: 1,
            start_time: time::Duration::new(0, 0),
            repetitions: 1,
            file: use_file(5),
            end_of_transmission: false,
            ascii_array: use_ascii_array(3),
            bit_sequence: use_bitsequence(3),
            sequence_count: 0,
            binary_msg: env::var("BINARY_MSG").is_ok(),
        };
        let mut word = String::new();
        let mut ch: u8 = 0;
        //for j in 0..6 { //version 2
        for _ in 0..50 {
            for i in 0..8 {
                ch = match bg.get_bit(&time::Duration::from_secs(0)) {
                    v => ch | v << (7 - i),
                };
            }
            word.push(ch as char);
            ch = 0;
        }
        //assert_eq!("hello!", word); //version 2, 2, 1
        assert_eq!("\0Automatic message.\0", word); //version 3, 3, 3
    }

    //ascii string is not necessary!

    // test if the file is just being read or also deleted to make sure that repititions work
    #[test]
    fn repetitions() {
        let mut bg = Bitstreamgenerator {
            id: 1,
            start_time: time::Duration::new(0, 0),
            repetitions: 2,
            file: use_file(4),
            end_of_transmission: false,
            ascii_array: use_ascii_array(3),
            bit_sequence: use_bitsequence(3),
            sequence_count: 0,
            binary_msg: env::var("BINARY_MSG").is_ok(),
        };
        let mut word = String::new();
        let mut ch: u8 = 0;
        //for j in 0..6 { //version 2
        for _ in 0..80 {
            for i in 0..8 {
                ch = match bg.get_bit(&time::Duration::from_secs(0)) {
                    v => ch | v << (7 - i),
                };
            }
            word.push(ch as char);
            ch = 0;
        }
        assert_eq!(
            "\0This text should be delivered. \r\nThis text should be delivered. \r\n\0",
            word
        );
    }
}
