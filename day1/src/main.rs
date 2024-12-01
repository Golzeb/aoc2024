use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_input(path: &'static str) -> (Vec<i64>, Vec<i64>) {
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

    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();

    file_reader
        .lines()
        .map(|line_res| match line_res {
            Ok(line) => {
                let mut temp = line
                    .split_whitespace()
                    //Assumption: data is always correct
                    .map(|f| i64::from_str_radix(f, 10).unwrap());

                let a = temp.next().unwrap();
                let b = temp.next().unwrap();

                Some((a, b))
            }
            Err(_) => None,
        })
        .for_each(|e| match e {
            Some(x) => {
                first_list.push(x.0);
                second_list.push(x.1);
            }
            None => {}
        });

    first_list.sort();
    second_list.sort();

    (first_list, second_list)
}

fn part1(first_list: &[i64], second_list: &[i64]) -> i64 {
    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(&a, &b)| a.abs_diff(b) as i64)
        .sum()
}

fn part2(first_list: &[i64], second_list: &[i64]) -> i64 {
    first_list
        .iter()
        .map(|e| (second_list.iter().filter(|&x| x == e).count() as i64) * e)
        .sum()
}

fn main() {
    let (first_list, second_list) = load_input("./day1/input.txt");

    println!("Part 1: {}", part1(&first_list, &second_list));
    println!("Part 2: {}", part2(&first_list, &second_list));
}
