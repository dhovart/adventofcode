use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    boards: Vec<Vec<Vec<i32>>>,
    drawn_numbers: Vec<i32>,
}

fn main() {
    match parse_input("./input.txt") {
        Ok(game) => println!("{:?}", game),
        Err(err) => println!("{}", err),
    };
}

fn part1(game: Game) -> i32 {
    0
}

fn parse_input<P>(filename: P) -> io::Result<Game>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();
    let mut drawn_numbers: Vec<i32> = vec![];
    if let Some(result) = lines.next() {
        drawn_numbers = result
            .unwrap()
            .split(',')
            .filter_map(|c| c.parse::<i32>().ok())
            .collect();
    }
    lines.next();
    let boards: Vec<String> = lines.into_iter().filter_map(|l| l.ok()).collect();
    let boards: Vec<Vec<Vec<i32>>> = boards.chunks(6)
        .map(|v|
                v.into_iter()
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.split(" ")
                            .filter_map(|s| s.parse::<i32>().ok())
                            .collect()
                        )
                    }
                })
                .collect())
        .collect();

    Ok(Game {
        boards,
        drawn_numbers,
    })
}
