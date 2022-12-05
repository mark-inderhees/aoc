use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    // let input = "./src/input.txt";
    let input = "./src/input_test.txt";
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
