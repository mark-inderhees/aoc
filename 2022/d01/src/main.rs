use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let mut max_cals = 0;
    if let Ok(lines) = read_lines("./src/input") {
        let mut current_cals = 0;
        for line in lines {
            if let Ok(x) = line {
                // println!("{}, {}", x, x.len());
                if let Ok(cals) =x.parse::<i32>() {
                    current_cals+=cals;
                }
                else {
                    if current_cals > max_cals{
                        max_cals = current_cals
                    }
                    current_cals = 0
                }
            }
        }
    }
    println!("{}", max_cals);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
