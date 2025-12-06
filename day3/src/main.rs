use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    const RADIX: u32 = 10;
    let mut power: u64 = 0;
    let filename = &args[1];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut line_power: u64 = 0;
            let line_of_battery: Vec<u32> = line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
            let mut max_index: usize = 0;
            for i in (0..12).rev() {
                let mut max_value: u32 = 0;
                let mut new_index: usize = max_index;
                for (index, value)  in line_of_battery[max_index..line_of_battery.len()-i].iter().enumerate() {
                    if *value > max_value {
                        max_value = *value;
                        new_index = index + max_index + 1;
                        //println!("found {} at {}", value, new_index)
                    }
                    //println!("line of battery {}", value);
                }
                line_power +=  max_value as u64 * 10_u64.pow(i.try_into().unwrap());
                max_index = new_index;
            }
            
            power += line_power;
            println!("max power is {}", line_power);
        }

    }
    println!("The total power is {}", power);
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
