use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day06.txt").unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut memory: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| (x, x))
        .collect();

    for line in lines {
        let xs: Vec<&str> = line.split_whitespace().collect();
        if xs[0] == "+" || xs[0] == "*" {
            let result = memory
                .iter()
                .zip(xs)
                .map(|((x, y), op)| match op {
                    "+" => x,
                    "*" => y,
                    _ => unreachable!(),
                })
                .sum::<usize>();
            println!("{result}");
            return;
        }

        let xs = xs.iter().map(|x| x.parse::<usize>().unwrap());
        memory.iter_mut().zip(xs).for_each(|((x, y), z)| {
            *x = *x + z;
            *y = *y * z;
        });
    }
}
