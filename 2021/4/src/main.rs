use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cell::RefCell;

#[derive(Debug)]
struct Cell {
    value: i32,
    drawn: bool,
}

impl Cell {
    fn new(value: i32) -> Cell {
        Cell {
            value,
            drawn: false,
        }
    }

    fn draw(&mut self) {
        self.drawn = true;
    }
}

#[derive(Debug)]
struct Board {
    value: Vec<Vec<RefCell<Cell>>>,
    winning: bool,
}

impl Board {
    fn new(value: Vec<Vec<RefCell<Cell>>>) -> Board {
        Board { value, winning: false }
    }

    fn has_winning_rows(&self) -> bool {
        self.value
            .iter()
            .any(|row| row.iter().all(|cell| cell.borrow().drawn))
    }

    fn has_winning_cols(&self) -> bool {
        for pos in 0..4 {
            let col: Vec<&RefCell<Cell>> = self.value.iter().map(|row| &row[pos]).collect();
            if col.iter().all(|cell| cell.borrow().drawn) {
                return true;
            }
        }
        false
    }

    fn get_undrawn_numbers_sum(&self) -> i32 {
        self.value.iter().fold(0, |sum, row| {
            let undrawn_sum = row.iter().fold(
                0,
                |sum, cell| {
                    let cell = cell.borrow();
                    if cell.drawn {
                        sum
                    } else {
                        sum + cell.value
                    }
                },
            );
            undrawn_sum + sum
        })
    }

    fn set_winning(&mut self) {
        self.winning = true
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<RefCell<Board>>,
    drawn_numbers: Vec<i32>,
}

impl Game {
    fn new(boards: Vec<RefCell<Board>>, drawn_numbers: Vec<i32>) -> Game {
        Game {
            boards,
            drawn_numbers,
        }
    }
    fn draw(&self, number: i32) -> () {
        self.boards.iter().filter(|b| !b.borrow().winning).for_each(|board| {
            board.borrow().value.iter().for_each(|row| {
                row.iter()
                    .filter(|c| c.borrow().value == number)
                    .for_each(|cell| {
                        cell.borrow_mut().draw();
                    });
            });
        })
    }

    fn query_winning_board(&self) -> Option<&RefCell<Board>> {
        let non_winning_boards: Vec<&RefCell<Board>> = self.boards.iter().filter(|board| !board.borrow().winning).collect();
        for i in 0..non_winning_boards.len() {
            let mut board = non_winning_boards[i].borrow_mut();
            if board.has_winning_rows() || board.has_winning_cols() {
                board.set_winning();
                return Some(&non_winning_boards[i]);
            }
        }
        None
    }

    fn get_drawn_number_at(&self, i: usize) -> i32 {
        self.drawn_numbers[i]
    }
}

fn main() {
    match parse_input("./input.txt") {
        Ok(game) => println!("{:?}", part1(&game)),
        Err(err) => println!("{}", err),
    };
}

fn part1(game: &Game) -> i32 {
    for i in 0..game.drawn_numbers.len() {
        let number = game.drawn_numbers[i];
        game.draw(number);
        if let Some(winning_board) = game.query_winning_board() {
            return number * winning_board.borrow().get_undrawn_numbers_sum();
        }
    }
    0
}

fn part2(game: &Game) -> i32 {
    let mut winning_boards = vec![];
    let mut number = 0;
    for i in 0..game.drawn_numbers.len() {
        number = game.get_drawn_number_at(i);
        game.draw(number);
        let possibly_winning_board: Option<&RefCell<Board>> = game.query_winning_board();
        if let Some(winning_board) = possibly_winning_board {
            winning_board.borrow_mut().set_winning();
            winning_boards.push(winning_board);
        }
    }
    if let Some(last_winning_board) = winning_boards.last() {
        let undrawn_numbers_sum = last_winning_board.borrow().get_undrawn_numbers_sum();
        return undrawn_numbers_sum * number
    }
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
    let boards: Vec<RefCell<Board>> = boards
        .chunks(6)
        .map(|v| {
            let value = v
                .into_iter()
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        let v: Vec<RefCell<Cell>> = s
                            .split(" ")
                            .filter_map(|s| {
                                if let Ok(value) = s.parse::<i32>() {
                                    Some(RefCell::new(Cell::new(value)))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        Some(v)
                    }
                })
                .collect();
            RefCell::new(Board::new(value))
        })
        .collect();

    Ok(Game::new(boards, drawn_numbers))
}
