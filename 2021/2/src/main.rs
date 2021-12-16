use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Position {
    x: i32,
    y: i32,
}

enum Move {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    match parse_input("./input.txt") {
        Ok(moves) => println!("{}", part2(moves)),
        Err(err) => println!("{}", err),
    };
}

fn part1(moves: Vec<Move>) -> i32 {
    let displacement = moves.iter().fold(Position { x: 0, y: 0}, |position, m|
        match m {
            | Move::Forward(x) => Position { x: position.x + x, y: position.y},
            | Move::Up(y) => Position { x: position.x, y: position.y - y},
            | Move::Down(y) => Position { x: position.x, y: position.y + y},
        }
    );
    displacement.x * displacement.y
}

fn part2(moves: Vec<Move>) -> i32 {
    let (displacement, _) = moves.iter().fold((Position { x: 0, y: 0}, 0), |(position, aim), m|
        match m {
            | Move::Forward(x) => {
                (Position { x: position.x + x, y: position.y + x * aim }, aim)
            },
            | Move::Up(y) => {
                (position, aim - y)
            },
            | Move::Down(y) => {
                (position, aim + y)
            },
        }
    );
    displacement.x * displacement.y
}

fn parse_input<P>(filename: P) -> io::Result<Vec<Move>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|line| {
        if let Ok(l) = line {
            if l.starts_with("forward") {
                return Some(Move::Forward(l[8..].parse::<i32>().ok()?))
            } else if l.starts_with("up") {
                return Some(Move::Up(l[3..].parse::<i32>().ok()?))
            } else if l.starts_with("down") {
                return Some(Move::Down(l[5..].parse::<i32>().ok()?))
            }
        }
        None
    }).collect())
}