use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input/day04.txt").unwrap();
    let mut map: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut result = 0;
    let mut stop = false;

    while !stop {
        stop = true;
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '.' {
                    continue;
                }

                let mut count = 0;
                let ii = i as isize;
                let jj = j as isize;
                for i in 0.max(ii - 1)..=(ii + 1).min(map.len() as isize - 1) {
                    for j in 0.max(jj - 1)..=(jj + 1).min(map[i as usize].len() as isize - 1) {
                        if map[i as usize][j as usize] == '@' {
                            count += 1;
                        }
                    }
                }

                if count <= 4 {
                    map[i][j] = '.';
                    stop = false;
                    result += 1;
                }
            }
        }
    }

    println!("{result}");
}
