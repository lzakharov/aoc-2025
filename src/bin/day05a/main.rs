use std::{
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn main() {
    let file = std::fs::File::open("input/day05.txt").unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let ranges: Vec<RangeInclusive<usize>> = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();
            start..=end
        })
        .collect();

    let result = lines
        .map(|line| line.parse().unwrap())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    println!("{result}");
}
