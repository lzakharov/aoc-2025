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

            // this algorithm can be improved by iterating over the halves of
            // the input numbers and it was my first correct solution, but i
            // decided to push a more compact one
            let mut sum = 0;
            for num in from..=to {
                let s = num.to_string();
                if s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..] {
                    sum += num;
                }
            }

            acc + sum
        });

    println!("{result}");
}
