use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dial: i32 = 50;
    let mut secret: i32 = 0;
    let filename = &args[1];
    let left_sequence = Regex::new(r"L(\d+)$").unwrap();
    let right_sequence = Regex::new(r"R(\d+)$").unwrap();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut calculated: i32 = 0;
            let mut number_of_zeros: i32 = 0;
            let mut round: i32 = 0;
            if left_sequence.is_match(&line) {
                let caps = left_sequence.captures(&line).unwrap();
                let value: i32 = caps[1].parse().unwrap();
                calculated = dial - value;
                while calculated < 0 {
                    calculated += 100;
                    number_of_zeros += 1;
                }

            }
            if right_sequence.is_match(&line) {
                let caps = right_sequence.captures(&line).unwrap();
                let value: i32 = caps[1].parse().unwrap();
                calculated = dial + value;
                while calculated >= 100 {
                    calculated -= 100;
                    round += 1;
                }
            }
            if dial == 0 && number_of_zeros > 0 {
                number_of_zeros -= 1;
            }
            dial = calculated;

            //println!("round of {} / 100 is {}", calculated, round);

            //println!("The dial made {} full rotations", round);
            if dial == 0 && round > 0 {
                round -= 1;
            }
            if dial == 0 {
                secret += 1;
            }
            secret += round;
            secret += number_of_zeros;
            print!("The dial is rotated {} to point at {}", line, dial);
            println!(" during this move, {} full rotations were made", round + number_of_zeros);
        }

    }
    println!("The secret code is {}", secret);
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
