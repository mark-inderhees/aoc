use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let shape_score = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let result_score = HashMap::from([("loss", 0), ("tie", 3), ("win", 6)]);
    let result_score2 = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let shape_finder = HashMap::from([
        (
            'X', // loss
            HashMap::from([('A', 'Z'), ('B', 'X'), ('C', 'Y')]),
        ),
        (
            'Y', // tie
            HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]),
        ),
        (
            'Z', // win
            HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]),
        ),
    ]);
    let results = HashMap::from([
        (
            'A', // rock
            HashMap::from([('X', "tie"), ('Y', "win"), ('Z', "loss")]),
        ),
        (
            'B', // paper
            HashMap::from([('X', "loss"), ('Y', "tie"), ('Z', "win")]),
        ),
        (
            'C', // scissors
            HashMap::from([('X', "win"), ('Y', "loss"), ('Z', "tie")]),
        ),
    ]);

    let mut total_score = 0;
    let mut total_score2 = 0;
    if let Ok(lines) = read_lines("./src/input") {
        for line in lines {
            if let Ok(l) = line {
                let them = l.chars().next().unwrap();
                let me = l.chars().last().unwrap();
                let result = results
                    .get(&them)
                    .expect("get them")
                    .get(&me)
                    .expect("get me");
                let s = shape_score.get(&me).expect("get shape score");
                let r = result_score.get(result).expect("get result score");
                let this_score = s + r;
                total_score += this_score;
                // println!("{} vs {}, {} {} {} {}", them, me, result, s, r, this_score);

                let my_needed = shape_finder
                    .get(&me)
                    .expect("get me 2")
                    .get(&them)
                    .expect("get them 2");
                let r2 = result_score2.get(&me).expect("get result score 2");
                let s2 = shape_score.get(&my_needed).expect("get shape score 2");
                let this_score2 = s2+r2;
                total_score2 += this_score2;
                println!("{} vs {}, {} {} {}", them, me, s2, r2, this_score2);
            }
        }
    }
    println!("{}", total_score);
    println!("{}", total_score2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
