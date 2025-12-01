use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day01.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    let result = lines
        .map(|line| {
            let x = line[1..].parse::<i32>().unwrap();
            if line.starts_with('R') { x } else { -x }
        })
        .scan(50, |current, x| {
            *current += x;
            Some(*current)
        })
        .filter(|current| current % 100 == 0)
        .count();

    println!("{result}");
}
