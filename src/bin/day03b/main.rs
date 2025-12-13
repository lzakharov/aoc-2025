use std::{
    collections::LinkedList,
    io::{BufRead, BufReader},
};

const N: usize = 12;

fn main() {
    let file = std::fs::File::open("input/day03.txt").unwrap();
    let result = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .fold(0, |acc, s| acc + joltage(&s));

    println!("{result}");
}

fn joltage(bank: &str) -> u64 {
    let batteries: Vec<char> = bank.chars().collect();
    let mut result = (0..N).collect::<LinkedList<usize>>();

    for i in N..bank.len() {
        result.push_back(i);
        for ((j, &left), &right) in result.iter().enumerate().zip(result.iter().skip(1)) {
            if batteries[left] < batteries[right] {
                let mut tail = result.split_off(j);
                tail.pop_front();
                result.append(&mut tail);
                break;
            }
        }
        if result.len() > N {
            result.pop_back();
        }
    }

    result.iter().fold(0, |acc, &i| {
        10 * acc + batteries[i].to_digit(10).unwrap() as u64
    })
}
