use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world! {} {}", 'a' as u32,'A' as u32);

    let mut score = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let len = l.len() / 2;
                let compartment1 = &l[..len];
                let compartment2 = &l[len..];
                println!("{} {} {} {}", l, len, compartment1, compartment2);
                for c in compartment1.chars() {
                    if compartment2.chars().any(|x| x == c) {
                        let s = match c {
                            'a'..='z' => c as i32 - 96,
                            _=> c as i32 - 64 + 26,
                        };
                        println!("{} {}", c, s);
                        score += s;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
