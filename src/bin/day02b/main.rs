use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day02.txt").unwrap();
    let result = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .fold(0, |acc, s| {
            let (from, to) = s.split_at(s.find('-').unwrap());
            let from = from.parse::<u64>().unwrap();
            let to = (&to[1..]).parse::<u64>().unwrap();

            let mut sum = 0;
            for num in from..=to {
                let s = num.to_string();
                for i in 1..=s.len() / 2 {
                    if s.len() % i == 0 && s[..i].repeat(s.len() / i) == s {
                        sum += num;
                        break;
                    }
                }
            }

            acc + sum
        });

    println!("{result}");
}
