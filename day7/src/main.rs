use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Equation {
    value: i64,
    operators: Vec<i64>,
}

fn load_input(path: &'static str) -> Vec<Equation> {
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

    let mut equations: Vec<Equation> = Vec::new();

    for line in file_reader.lines().filter_map(Result::ok) {
        let (value_str, operators_str) = line.split_once(':').unwrap();
        let value = value_str.parse::<i64>().unwrap();
        let operators: Vec<i64> = operators_str
            .split_whitespace()
            .map(str::parse::<i64>)
            .filter_map(Result::ok)
            .collect();

        equations.push(Equation { value, operators });
    }

    equations
}

fn is_solvable(
    value: i64,
    operations: &[fn(i64, i64) -> i64],
    current_value: i64,
    index: usize,
    operators: &[i64],
) -> bool {
    if index < operators.len() {
        operations.iter().any(|e| {
            is_solvable(
                value,
                operations,
                e(current_value, operators[index]),
                index + 1,
                operators,
            )
        })
    } else {
        value == current_value
    }
}

fn part1(equations: &[Equation]) -> i64 {
    let operations: Vec<fn(i64, i64) -> i64> =
        vec![|a: i64, b: i64| -> i64 { a + b }, |a: i64, b: i64| -> i64 {
            a * b
        }];

    equations
        .iter()
        .filter(|e| is_solvable(e.value, &operations, e.operators[0], 1, &e.operators))
        .map(|e| e.value)
        .sum()
}

fn part2(equations: &[Equation]) -> i64 {
    let operations: Vec<fn(i64, i64) -> i64> = vec![
        |a: i64, b: i64| -> i64 { a + b },
        |a: i64, b: i64| -> i64 { a * b },
        |a: i64, b: i64| -> i64 { a * 10i64.pow(((b as f32).log10() + 1.0).floor() as u32) + b },
    ];

    equations
        .iter()
        .filter(|e| is_solvable(e.value, &operations, e.operators[0], 1, &e.operators))
        .map(|e| e.value)
        .sum()
}

fn main() {
    let equations = load_input("./day7/input.txt");

    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));
}
