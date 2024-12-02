use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_input(path: &'static str) -> Vec<Vec<u64>> {
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

    file_reader
        .lines()
        .filter_map(|line_res| match line_res {
            Ok(line) => {
                Some(
                    line.split_whitespace()
                        //Assumption: data is always correct
                        .map(|level| u64::from_str_radix(level, 10).unwrap())
                        .collect::<Vec<u64>>(),
                )
            }
            Err(_) => None,
        })
        .collect()
}

fn part1(reports: &[Vec<u64>]) -> u64 {
    reports
        .iter()
        .map(|levels| {
            let increasing = levels[0] < levels[1];
            levels.iter().zip(levels.iter().skip(1)).any(|(&a, &b)| {
                let diff = a.abs_diff(b);
                diff > 3 || diff == 0 || (a < b) != increasing
            })
        })
        .filter(|&e| !e)
        .count() as u64
}

fn part2(reports: &[Vec<u64>]) -> u64 {
    reports
        .iter()
        .map(|levels| {
            let mut values: Vec<bool> = Vec::new();

            let increasing = levels[0] < levels[1];
            let value = levels.iter().zip(levels.iter().skip(1)).any(|(&a, &b)| {
                let diff = a.abs_diff(b);
                diff > 3 || diff == 0 || (a < b) != increasing
            });

            values.push(value);

            for i in 0..levels.len() {
                let mut temp = levels.clone();
                temp.remove(i);

                let increasing = temp[0] < temp[1];
                let value = temp.iter().zip(temp.iter().skip(1)).any(|(&a, &b)| {
                    let diff = a.abs_diff(b);
                    diff > 3 || diff == 0 || (a < b) != increasing
                });

                values.push(value);
            }

            values.iter().any(|&e| !e)
        })
        .filter(|&e| e)
        .count() as u64
}

fn main() {
    let reports = load_input("./day2/input.txt");

    println!("Part 1: {}", part1(&reports));
    println!("Part 2: {}", part2(&reports));
}
