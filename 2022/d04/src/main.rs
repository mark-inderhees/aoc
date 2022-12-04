use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let mut count = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let mut start = "";
                let elves: Vec<&str> = l.split(",").collect();
                let r1: Vec<&str> = elves[0].split("-").collect();
                let r2: Vec<&str> = elves[1].split("-").collect();
                let r1_0: u32 = r1[0].parse().unwrap();
                let r1_1: u32 = r1[1].parse().unwrap();
                let r2_0: u32 = r2[0].parse().unwrap();
                let r2_1: u32 = r2[1].parse().unwrap();
                // if r1[0] == r2[0] && r1[1] == r2[1] {
                if r1_0 <= r2_0 && r1_1 >= r2_1 {
                    count += 1;
                    start = "*";
                } else if r2_0 <= r1_0 && r2_1 >= r1_1 {
                    count += 1;
                    start = "*";
                }
                println!(
                    "{} {}-{} {}-{} {} {}",
                    l, r1_0, r1_1, r2_0, r2_1, count, start
                );
            }
        }
    }
    println!("count {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
