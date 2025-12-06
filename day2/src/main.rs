use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut secret: i64 = 0;
    let filename = &args[1];
    let interval = Regex::new(r"(\d+)-(\d+)").unwrap();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let list: Vec<&str> = line.split(",").collect();
            for elem in list {
                let caps = interval.captures(elem).unwrap();
                let start: i64  = caps[1].parse().unwrap();
                let end: i64 = caps[2].parse().unwrap();
                //println!("{}-{}", start, end);
                for number in start..=end {
                    
                    let num_in_str = number.to_string();
                    let len = num_in_str.len() as usize;
                    //println!("Checking number: {}, {} and {}", num_in_str, &num_in_str[0..len/2], &num_in_str[len/2..len]);
                    for i in 2..len+1 {
                        let mut match_str: bool = true;
                        if len % i != 0 {
                            continue;
                        }
                        let chunk_size = len / i;
                        //println!("chunk size of {}", chunk_size);
                        for j in 0..i-1 {
                            
                            if num_in_str[j*chunk_size..(j+1)*chunk_size] != num_in_str[(j+1)*chunk_size..(j+2)*chunk_size] {
                                match_str = false;
                                //println!("{} no matching to {}", &num_in_str[j*chunk_size..(j+1)*chunk_size], &num_in_str[(j+1)*chunk_size..(j+2)*chunk_size]);
                            }
                        }
                        if match_str {
                            println!("Found {}", number);
                            secret += number;
                            break
                        } else {
                            //println!("not working {}", number);
                        }
                    }
                }
            }
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
