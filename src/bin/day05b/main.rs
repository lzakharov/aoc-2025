use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day05.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let mut ranges: Vec<(usize, usize)> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_at(line.find('-').unwrap());
            let start = start.parse::<usize>().unwrap();
            let end = end[1..].parse::<usize>().unwrap();
            (start, end)
        })
        .collect();

    ranges.sort();

    let mut result: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());

    ranges.iter().for_each(|&x| {
        for y in result.iter_mut() {
            if contains(y.0, y.1, x.0) {
                *y = (y.0, x.1.max(y.1));
                return;
            }
            if contains(y.0, y.1, x.1) {
                *y = (x.0.min(y.0), y.1);
                return;
            }
        }
        result.push(x);
    });

    let total = result.iter().map(|&r| r.1 - r.0 + 1).sum::<usize>();
    println!("{total}");
}

fn contains(start: usize, end: usize, x: usize) -> bool {
    start <= x && x <= end
}
