use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day03.txt").unwrap();
    let result = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .fold(0, |acc, s| acc + joltage(&s));

    println!("{result}");
}

fn joltage(bank: &str) -> u32 {
    let x = bank[..bank.len() - 1].chars().max().unwrap();
    let y = bank[bank.find(x).unwrap() + 1..].chars().max().unwrap();
    10 * (x.to_digit(10).unwrap()) + y.to_digit(10).unwrap()
}
