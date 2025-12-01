use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day01.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    let result = lines
        .map(|line| {
            let x = line[1..].parse::<i32>().unwrap();
            if line.starts_with('R') { x } else { -x }
        })
        .fold((50, 0), |(current, result), x| {
            let mut zeros = (x / 100).abs();
            let d = (x % 100).abs();
            if (x > 0 && current + d >= 100) || (x < 0 && current != 0 && current - d <= 0) {
                zeros += 1;
            }
            ((current + x).rem_euclid(100), result + zeros)
        })
        .1;

    println!("{result}");
}
