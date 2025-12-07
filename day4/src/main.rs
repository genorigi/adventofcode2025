use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut movable_rolls: i32 = 1;
    let mut total:i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            matrix.push(line.chars().collect());
        }
    }
    while (movable_rolls != 0) {
        let mut temp_matrix = matrix.clone();
        movable_rolls = 0;
        for (y, line) in matrix.clone().iter().enumerate() {
            for (x, elem) in line.iter().enumerate() {
                //println!("processing {}:{}", x, y);
                if *elem == '@' && score(matrix.clone(), x as i32, y as i32) < 4 {
                    print!("x");
                    movable_rolls += 1;
                    matrix[y as usize][x as usize] = 'x';
                }
                else {
                    print!("{}", elem);
                }
            }
            println!("");
        }
        total += movable_rolls;
    }
    println!("the number of movable rolls is {}", total);
}

// return the number of rolls of paper next to the specified x,y coordinate
fn score(
    grid: Vec<Vec<char>>,
    x: i32,
    y: i32
) -> i32 {
    let mut score: i32 = 0;
    let y_coordinates = [ y-1, y, y+1];
    let x_coordinates = [ x-1, x, x+1];
    
    for j in y_coordinates {
        if j < 0 || j >= grid.len() as i32 {
            continue;
        }

        for i in x_coordinates {
            if j == y && i == x {
                continue;
            }
            if i >= 0 && i < grid[j as usize].len() as i32 {
                //println!("checking {},{}: {}", i, j, grid[j as usize][i as usize] );
                if grid[j as usize][i as usize] == '@' {
                    score += 1
                }
            }
        }
    }
    //println!("score for {}:{} is {}", x, y, score);
    return score;
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
