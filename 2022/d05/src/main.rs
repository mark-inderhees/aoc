use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    // let mut v1 = vec!['N', 'Z'];
    // v1.reverse();
    // let mut v2 = vec!['D', 'C', 'M'];
    // v2.reverse();
    // let mut v3 = vec!['P'];
    // v3.reverse();
    // let mut lists = vec![v1, v2, v3];

    let mut v1 = vec!['S', 'P', 'W', 'N', 'J', 'Z'];
    let mut v2 = vec!['T', 'S', 'G'];
    let mut v3 = vec!['H', 'L', 'R', 'Q', 'V'];
    let mut v4 = vec!['D', 'T', 'S', 'V'];
    let mut v5 = vec!['J', 'M', 'B', 'D', 'T', 'Z', 'Q'];
    let mut v6 = vec!['L', 'Z', 'C', 'D', 'J', 'T', 'W', 'M'];
    let mut v7 = vec!['J', 'T', 'G', 'W', 'M', 'P', 'L'];
    let mut v8 = vec!['H', 'Q', 'F', 'B', 'T', 'M', 'G', 'N'];
    let mut v9 = vec!['W', 'Q', 'B', 'P', 'C', 'G', 'D', 'R'];
    v1.reverse();
    v2.reverse();
    v3.reverse();
    v4.reverse();
    v5.reverse();
    v6.reverse();
    v7.reverse();
    v8.reverse();
    v9.reverse();
    let mut lists = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let orders: Vec<&str> = l.split(" ").collect();
                let count: usize = orders[1].parse().unwrap();
                let mut source: usize = orders[3].parse().unwrap();
                source -= 1;
                let mut dest: usize = orders[5].parse().unwrap();
                dest -= 1;
                println!("move {} from {} to {}", count, source, dest);
                for _ in 0..count {
                    let val = lists[source].pop().expect("bob");
                    lists[dest].push(val);
                }
            }
        }
    }

    for mut bob in lists {
        print!("{}", bob.pop().expect("moo"));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
