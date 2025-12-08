use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str;

#[derive(Debug, Clone, Copy)]
struct Interval {
    min: u64,
    max: u64
}

impl Interval {
    fn includes(&self, x: u64) -> bool {
        return x >= self.min && x <= self.max
    }
    fn new(x: u64, y:u64) -> Self {
        if x >= y {
            return Self { min: y, max: x};
        } else {
            return Self { min: x, max: y};
        }
    }
    fn intersect(&self, another_interval: Interval) -> Option<Interval> {
        if self.min > another_interval.max || self.max < another_interval.min {
            return None;
        } else {
            return Some(Interval::new(min(self.min, another_interval.min), max(self.max, another_interval.max)))
        }
    }
}

fn min (x: u64, y:u64) -> u64 {
    if x > y {
        return y;
    } else {
        return x;
    }
}

fn max (x: u64, y:u64) -> u64 {
    if x > y {
        return x;
    } else {
        return y;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut ranges: Vec<Interval> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    let mut total_part1: i32 = 0;
    let mut total_part2: u64 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut is_interval = true;
        for line in lines.flatten() {
            if line == "" {
                is_interval = false;
                continue;
            }
            if is_interval {
                let interval_string: Vec<&str> = line.split("-").collect();
                let start: u64  = interval_string[0].parse().unwrap();
                let end: u64 = interval_string[1].parse().unwrap();
                ranges.push(Interval::new(start, end))
            }
            if !is_interval {
                let item: u64 = line.parse().unwrap();
                for range in ranges.clone() {
                    if range.includes(item) {
                        total_part1 += 1;
                        break;
                    }
                }
            }
        }
    }
    // for part2 we will deal with ranges intersections
    let mut converged: bool = false;
    let mut new_ranges: Vec<Interval> = Vec::new();
    while ranges.len() > 0 {
        let range = ranges.pop();
        //println!("loop");
        for range in ranges.clone() {
            //println!("{}-{}", range.min, range.max);
        }
        //println!("new ranges");
        for range in new_ranges.clone() {
            //println!("{}-{}", range.min, range.max);
        }
        let mut found_intersection = false;
        for (index, test_range) in ranges.clone().iter().enumerate() {
            let new_range = range.unwrap().intersect(test_range.clone());
            //println!("comparing range1: {}-{} with range2 {}-{}", range.min, range.max, test_range.min, test_range.max);
            if new_range.is_some() {
                //println!("found intersection");
                let extracted_range = new_range.unwrap();
                ranges[index] = extracted_range;
                //println!("found intersection, storing new range:{}-{} in index {}", extracted_range.min, extracted_range.max, index);
                found_intersection = true;
                break;
            }
        }
        if !found_intersection {
            new_ranges.push(range.unwrap());
        }
    }
    for range in new_ranges.clone() {
        // add all ranges size.
        //println!("found range {}-{}", range.min, range.max);
        total_part2 += (range.max - range.min) + 1
    }
    println!("the number of fresh ingredients is {}", total_part1);
    println!("the number of potential ingredients is {}", total_part2);
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
