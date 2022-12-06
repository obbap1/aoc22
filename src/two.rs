use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use typedmap::{TypedMap, TypedMapKey};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Choice{name: String}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Details{wins_against: [String; 3]}

pub fn my_score(){
    let file = match File::open("src/input/two.txt") {
        Err(e) => panic!("{}",e),
        Ok(f) => f 
    };

    let buf_reader = BufReader::new(file);
    
    let score = 0;

    for line in buf_reader.lines() {
        let curr_line = line.expect("unable to read line from buffer");
        let split = curr_line.split(" ");
        let vec: Vec<&str> = split.collect();
        let (opponents_choice, my_choice) = (vec[0], vec[1]);

    }
}