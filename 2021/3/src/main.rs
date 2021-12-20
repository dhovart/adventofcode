use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match parse_input("./input.txt") {
        Ok(rows) => println!("{}", part2(rows)),
        Err(err) => println!("{}", err),
    };
}

fn part1(rows: Vec<Vec<u32>>) -> i32 {
    let row_length = rows[0].len();
    let rows_as_ref: &Vec<&Vec<u32>>  = &rows.iter().map(|v| v.as_ref()).collect();
    let mut result = vec![];
    for i in 0..row_length {
        result.push(find_most_common_bit_at_position(i, rows_as_ref));
    }
    let gamma = binary_vec_to_i32(&result);
    let epsilon = !gamma & 0b00000000000000000000111111111111;
    gamma * epsilon
}

fn part2(rows: Vec<Vec<u32>>) -> i32 {
    let unfiltered_rows = rows.iter().map(|v| v.as_ref()).collect();
    let oxygen_generator_rating = recursively_filter_using_most_or_least_common_bit(0, unfiltered_rows, true);
    let oxygen_generator_rating = binary_vec_to_i32(&oxygen_generator_rating[0]);
    let unfiltered_rows = rows.iter().map(|v| v.as_ref()).collect();
    let co2_scrubber_rating = recursively_filter_using_most_or_least_common_bit(0, unfiltered_rows, false);
    let co2_scrubber_rating = binary_vec_to_i32(&co2_scrubber_rating[0]);
    oxygen_generator_rating * co2_scrubber_rating
}


fn find_most_or_least_common_bit_at_position(pos: usize, rows: &Vec<&Vec<u32>>, most_common: bool) -> u32 {
    let col: Vec<u32> = rows.iter().map(|row| row[pos]).collect();
    let (total_0, total_1) = col.iter().fold((0, 0), |(total_0, total_1), cell| {
        if *cell == 0 {
            (total_0 + 1, total_1)
        } else {
            (total_0, total_1 + 1)
        }
    });

    let should_return_0: bool = if most_common {
        total_0 > total_1
    } else {
        total_0 < total_1 || total_0 == total_1
    };

    if should_return_0 {
        0
    } else {
        1
    }
}

fn find_most_common_bit_at_position(pos: usize, rows: &Vec<&Vec<u32>>) -> u32 {
    find_most_or_least_common_bit_at_position(pos, rows, true)
}

fn binary_vec_to_i32(vec: &Vec<u32>) -> i32 {
    let binary_string: String = vec
        .iter()
        .filter_map(|c| char::from_digit(*c, 10))
        .collect();
    i32::from_str_radix(&binary_string, 2).unwrap()
}


fn recursively_filter_using_most_or_least_common_bit<'a>(pos: usize, rows: Vec<&'a Vec<u32>>, most_common: bool) -> Vec<&'a Vec<u32>> {
    if rows.len() == 1 {
        return rows
    }
    let filtered_rows: Vec<&Vec<u32>> = rows.iter()
        .filter(|row| row[pos] == find_most_or_least_common_bit_at_position(pos, &rows, most_common))
        .copied()
        .collect();
    
    recursively_filter_using_most_or_least_common_bit(
        pos + 1,
        filtered_rows,
        most_common
    )
}
fn parse_input<P>(filename: P) -> io::Result<Vec<Vec<u32>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let rows = io::BufReader::new(file)
        .lines()
        .filter_map(|line| {
            if let Ok(l) = line {
                let row: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
                return Some(row);
            }
            None
        })
        .collect();
    Ok(rows)
}
