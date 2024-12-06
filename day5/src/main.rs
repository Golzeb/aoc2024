use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Data {
    rules: HashMap<i64, Vec<i64>>,
    updates: Vec<Vec<i64>>,
}

impl Data {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
            updates: Vec::new(),
        }
    }
}

fn load_input(path: &'static str) -> Data {
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

    let mut data = Data::new();

    let mut rules = true;

    for line in file_reader.lines().filter_map(|e| e.ok()) {
        if rules {
            if line == "" {
                rules = false;
            } else {
                let rule: Vec<i64> = line.split("|").map(|e| e.parse::<i64>().unwrap()).collect();

                data.rules.entry(rule[0]).or_default().push(rule[1]);
            }
        } else {
            let update: Vec<i64> = line.split(",").map(|e| e.parse::<i64>().unwrap()).collect();

            data.updates.push(update);
        }
    }

    data
}

fn part1(data: &Data) -> i64 {
    let mut previous_pages: Vec<i64> = Vec::new();

    data.updates
        .iter()
        .filter_map(|update| {
            previous_pages.clear();
            let found = update.iter().any(|page| {
                let value = data
                    .rules
                    .get(page)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|e| previous_pages.contains(e));

                previous_pages.push(*page);

                value
            });

            if !found {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part2(data: &Data) -> i64 {
    let mut previous_pages: Vec<i64> = Vec::new();

    data.updates
        .iter()
        .filter_map(|update| {
            previous_pages.clear();
            let found = update.iter().any(|page| {
                let value = data
                    .rules
                    .get(page)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|e| previous_pages.contains(e));

                previous_pages.push(*page);

                value
            });

            if !found {
                None
            } else {
                let mut fixed_update: Vec<i64> = Vec::new();

                update.iter().for_each(|page| {
                    let earliest_occurence = data
                        .rules
                        .get(page)
                        .unwrap_or(&Vec::new())
                        .iter()
                        .fold(fixed_update.len(), |earliest_occurence, rule| {
                            if let Some(position) = fixed_update.iter().position(|e| e == rule) {
                                if position < earliest_occurence {
                                    position
                                } else {
                                    earliest_occurence
                                }
                            } else {
                                earliest_occurence
                            }
                        });

                    fixed_update.insert(earliest_occurence, *page);
                });

                Some(fixed_update[fixed_update.len() / 2])
            }
        })
        .sum()
}

fn main() {
    let data = load_input("./day5/input.txt");

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
