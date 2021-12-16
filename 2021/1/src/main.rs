use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match parse_input("./input.txt") {
        Ok(values) => println!("{}", part2(values)),
        Err(err) => println!("{}", err),
    };
}

fn part1(values: Vec<i32>) -> i32 {
    let depth = 0;
    let (_, increases) = values.iter().fold((&depth, -1), |(prev_depth, increases), depth| {
        if depth > &prev_depth {
            (depth, increases + 1)
        } else {
            (depth, increases)
        }
    });
    increases
}

fn part2(values: Vec<i32>) -> i32 {
    let chunk_size = 3;
    let mut sums = Vec::new();
    for i in 0..chunk_size {
        let values: Vec<&i32> = values.iter().skip(i).collect();
        let values: Vec<&[&i32]> = values.chunks(chunk_size).collect();
        let values: Vec<i32> = values.iter().filter_map(|v| {
            if (v.len()) < chunk_size {
                None
            } else {
                Some(v.iter().copied().sum::<i32>())
            }
        }).collect();
        sums.push(values);
    }

    let mut increases = 0;
    for i in 0..chunk_size {
        let i1 = sums[i].iter();
        let i2: Vec<&i32> = if i < chunk_size - 1 {
            sums[i + 1].iter().collect()
        } else {
            sums[0].iter().skip(1).collect()
        };
        increases += i1.zip(i2).filter(|&(a, b)| b > a).count();
    }
    increases as i32
}

fn parse_input<P>(filename: P) -> io::Result<Vec<i32>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|line| {
        if let Ok(l) = line {
            return Some(l.parse::<i32>().ok()?)
        }
        None
    }).collect())
}