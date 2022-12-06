use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn total_calories() -> i32 {
    let file = match File::open("src/input/one.txt") {
        Err(e) => panic!("{}",e),
        Ok(f) => f 
    };

    let buf_reader = BufReader::new(file);

    let mut list = Vec::new();
    let mut fin = Vec::new();

    for line in buf_reader.lines() {
        let curr_line = line.expect("unable to read line from buffer");
        if curr_line.is_empty() {
            let sum: i32 = list.iter().sum();
            fin.push(sum);
            list.clear();
        }else{
            let curr_int: i32 = curr_line.parse().expect("invalid int");
            list.push(curr_int);
        }
    }

    fin.sort_by(|a, b|b.cmp(a));
    fin[..3].iter().sum()
}