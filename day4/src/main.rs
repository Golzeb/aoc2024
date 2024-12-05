mod structs;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use structs::Matrix;

fn load_input(path: &'static str) -> Matrix<char> {
    let file = match File::open(path) {
        Ok(x) => x,
        Err(err) => {
            panic!(
                "Make sure that you copied your input to the day directory!\nError: {}",
                err
            );
        }
    };

    let file_reader = BufReader::new(file);

    let data: Vec<String> = file_reader
        .lines()
        .filter_map(|line_res| line_res.ok())
        .collect();

    let columns = data[0].len();
    let rows = data.len();

    Matrix::new(rows, columns).with_data(data.join("").chars().collect())
}

fn part1(input: &Matrix<char>) -> i64 {
    let mut count = 0;

    let mut current_index = 0;
    while let Some(row) = input.row(current_index) {
        let row = row.iter().collect::<String>();
        count += row.matches("XMAS").count() as i64;
        count += row.matches("SAMX").count() as i64;

        current_index += 1;
    }

    let mut current_index = 0;
    while let Some(column) = input.column(current_index) {
        let column = column.iter().collect::<String>();
        count += column.matches("XMAS").count() as i64;
        count += column.matches("SAMX").count() as i64;

        current_index += 1;
    }

    let mut current_index = 0;
    while let Some(left_diagonal) = input.left_diagonal(current_index) {
        let left_diagonal = left_diagonal.iter().collect::<String>();
        count += left_diagonal.matches("XMAS").count() as i64;
        count += left_diagonal.matches("SAMX").count() as i64;

        current_index += 1;
    }

    let mut current_index = 0;
    while let Some(right_diagonal) = input.right_diagonal(current_index) {
        let right_diagonal = right_diagonal.iter().collect::<String>();
        count += right_diagonal.matches("XMAS").count() as i64;
        count += right_diagonal.matches("SAMX").count() as i64;

        current_index += 1;
    }

    count
}

fn part2(input: &Matrix<char>) -> i64 {
    let mut count = 0;

    for i in 0..input.data().len() {
        if input.data()[i] == 'A'
            && i % input.column_count() != 0
            && i % input.column_count() != input.column_count() - 1
            && i / input.row_count() != 0
            && i / input.row_count() != input.row_count() - 1
        {
            let left_diagonal_start = i - input.column_count() - 1;
            let right_diagonal_start: usize = i - input.column_count() + 1;

            let right_diagonal = input
                .right_diagonal_at_index(right_diagonal_start)
                .unwrap()
                .iter()
                .collect::<String>();
            let left_diagonal = input
                .left_diagonal_at_index(left_diagonal_start)
                .unwrap()
                .iter()
                .collect::<String>();

            if (right_diagonal.starts_with("MAS") || right_diagonal.starts_with("SAM"))
                && (left_diagonal.starts_with("MAS") || left_diagonal.starts_with("SAM"))
            {
                count += 1
            }
        }
    }

    count
}

fn main() {
    let input = load_input("./day4/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
