use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cell::RefCell;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

#[derive(Debug)]
struct Game {
    lines: Vec<Line>,
    board: Vec<Vec<RefCell<i32>>>,
}

impl Game {
    fn process_lines(&self) {
        self.lines
            .iter()
            .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
            .for_each(|line| self.cross_board(line));
    }

    fn cross_board(&self, line: &Line) {
        let start_x = cmp::min(line.a.x, line.b.x);
        let end_x = cmp::max(line.a.x, line.b.x);
        let start_y = cmp::min(line.a.y, line.b.y);
        let end_y = cmp::max(line.a.y, line.b.y);

        if start_x != end_x {
            self.board[start_y][start_x..=end_x]
                .iter()
                .for_each(|cell| {
                    let mut cell = cell.borrow_mut();
                    *cell = *cell + 1;
                });
        } else {
            let col: Vec<&RefCell<i32>> = self.board.iter()
                .map(|row| &row[start_x])
                .collect();
            col[start_y..=end_y].iter().for_each(|cell| {
                    let mut cell = cell.borrow_mut();
                    *cell = *cell + 1;
                });
        }
    }

    fn find_total_overlaps(&self) -> i32 {
        self.board.iter().fold(0, |sum, row| {
            sum + row.iter().fold(0, |sum, cell| {
                let cell = cell.borrow();
                if *cell >= 2 {
                    sum + 1
                } else {
                    sum
                }
            })
        })
    }
}

fn main() {
    match parse_input("./input.txt") {
        Ok(game) => println!("{}", part1(&game)),
        Err(err) => println!("{}", err),
    };
}

fn part1(game: &Game) -> i32 {
    game.process_lines();
    game.find_total_overlaps()
}

fn extract_line_info(line: &str) -> Option<Line> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    }
    if let Some(captures) = RE.captures(line) {
        if captures.len() < 4 {
            return None;
        } else {
            return Some(Line {
                a: {
                    Point {
                        x: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        y: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    }
                },
                b: {
                    Point {
                        x: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                        y: captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    }
                },
            });
        }
    }
    None
}

fn parse_input<P>(filename: P) -> io::Result<Game>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let lines: Vec<Line> = lines
        .into_iter()
        .filter_map(|l| l.ok())
        .filter_map(|l| extract_line_info(&l))
        .collect();
    let board_dimensions = lines.iter().fold((0, 0), |(w, h), line| {
        let w = cmp::max(line.a.x + 1, w);
        let w = cmp::max(line.b.x + 1, w);
        let h = cmp::max(line.a.y + 1, h);
        let h = cmp::max(line.b.y + 1, h);
        (w, h)
    });
    let board = vec![vec![RefCell::new(0); board_dimensions.0]; board_dimensions.1];
    Ok(Game { lines, board })
}
