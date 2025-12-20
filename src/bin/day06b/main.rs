use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day06.txt").unwrap();
    let worksheet: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut total = 0;
    let mut current = 0;
    let mut op = '+';

    for col in 0..worksheet[0].len() {
        let ch = worksheet[worksheet.len() - 1][col];
        if !ch.is_whitespace() {
            total += current;
            op = ch;
            current = if op == '+' { 0 } else { 1 };
        }

        let mut num = 0;
        let mut empty = true;
        for row in 0..worksheet.len() - 1 {
            let ch = worksheet[row][col];
            if ch.is_whitespace() {
                continue;
            }
            empty = false;
            let digit = ch as usize - '0' as usize;
            num = 10 * num + digit;
        }

        if empty {
            continue;
        }

        match op {
            '+' => current += num,
            '*' => current *= num,
            _ => unreachable!(),
        }
    }

    total += current;

    println!("{total}")
}
