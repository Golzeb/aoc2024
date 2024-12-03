use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
enum Ops {
    Mul { x: i64, y: i64 },
    Do,
    Dont,
}

#[derive(Clone, Copy)]
enum ParserPattern {
    String(&'static str),
    Number(u64),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PatternMatch {
    Continue,
    Abort,
    DoneEarly,
    Done,
}

impl ParserPattern {
    fn match_pattern(&self, str: &str) -> PatternMatch {
        match self {
            ParserPattern::String(expected_str) => {
                if *expected_str == str {
                    PatternMatch::Done
                } else if expected_str.starts_with(str) {
                    PatternMatch::Continue
                } else {
                    PatternMatch::Abort
                }
            }
            ParserPattern::Number(max_length) => {
                let all_digits = str.chars().all(|e| e.is_digit(10));

                if !all_digits {
                    if str.len() > 1 {
                        PatternMatch::DoneEarly
                    } else {
                        PatternMatch::Abort
                    }
                } else if str.len() as u64 == *max_length {
                    PatternMatch::Done
                } else if (str.len() as u64) < *max_length {
                    PatternMatch::Continue
                } else {
                    PatternMatch::Abort
                }
            }
        }
    }
}

fn load_input(path: &'static str) -> String {
    let file = match File::open(path) {
        Ok(x) => x,
        Err(err) => {
            panic!(
                "Make sure that you copied your input to the day directory!\nError: {}",
                err
            );
        }
    };

    let mut file_reader = BufReader::new(file);

    let mut out = String::new();

    //Hmmm?
    let _ = file_reader.read_to_string(&mut out).unwrap();

    out
}

fn parse_input(data: &str) -> Vec<Ops> {
    const PATTERN_MUL: [ParserPattern; 5] = [
        ParserPattern::String("mul("),
        ParserPattern::Number(3),
        ParserPattern::String(","),
        ParserPattern::Number(3),
        ParserPattern::String(")"),
    ];

    const PATTERN_DO: [ParserPattern; 1] = [ParserPattern::String("do()")];
    const PATTERN_DONT: [ParserPattern; 1] = [ParserPattern::String("don't()")];

    const PATTERNS: [&[ParserPattern]; 3] = [&PATTERN_MUL, &PATTERN_DO, &PATTERN_DONT];

    let data_chars: Vec<char> = data.chars().collect();

    let mut out: Vec<Ops> = Vec::new();

    let mut current_value = String::new();
    let mut temp = String::new();

    let mut current_state = 0;
    let mut current_index = 0;

    while current_index < data.len() {
        temp.push(data_chars[current_index]);

        let matches: Vec<PatternMatch> = PATTERNS
            .iter()
            .map(|e| {
                if e.len() > current_state {
                    e[current_state].match_pattern(&temp)
                } else {
                    PatternMatch::Abort
                }
            })
            .collect();

        if matches.iter().any(|&e| e == PatternMatch::Continue) {
            current_index += 1;
        } else if matches.iter().any(|&e| e == PatternMatch::Done) {
            current_state += 1;

            current_value += &temp;
            temp.clear();

            current_index += 1;
        } else if matches.iter().any(|&e| e == PatternMatch::DoneEarly) {
            temp.pop();

            current_state += 1;

            current_value += &temp;
            temp.clear();
        } else {
            current_state = 0;

            temp.clear();
            current_value.clear();

            current_index += 1;
        }

        if !matches.iter().any(|&e| e == PatternMatch::Continue)
            && matches
                .iter()
                .enumerate()
                .filter(|&(index, &e)| {
                    e == PatternMatch::Done && PATTERNS[index].len() == current_state
                })
                .count()
                == 1
        {
            let index = matches
                .iter()
                .enumerate()
                .filter_map(|(index, &v)| {
                    if v == PatternMatch::Done {
                        Some(index)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap();

            match index {
                0 => {
                    current_value = current_value.replace("mul(", "");
                    current_value = current_value.replace(")", "");

                    let mut it = current_value
                        .split(",")
                        .map(|e| i64::from_str_radix(e, 10).unwrap());

                    out.push(Ops::Mul {
                        x: it.next().unwrap(),
                        y: it.next().unwrap(),
                    });
                }
                1 => out.push(Ops::Do),
                2 => out.push(Ops::Dont),
                _ => {
                    unreachable!()
                }
            }

            current_value.clear();
            current_state = 0;
        }
    }

    out
}

fn part1(ops: &[Ops]) -> i64 {
    ops.iter()
        .filter_map(|e| match e {
            Ops::Mul { x, y } => Some(x * y),
            _ => None,
        })
        .sum()
}

fn part2(ops: &[Ops]) -> i64 {
    let mut enabled = true;
    ops.iter()
        .filter_map(|e| {
            if enabled {
                match e {
                    Ops::Mul { x, y } => Some(x * y),
                    Ops::Dont => {
                        enabled = false;
                        None
                    }
                    _ => None,
                }
            } else {
                match e {
                    Ops::Do => {
                        enabled = true;
                        None
                    }
                    _ => None,
                }
            }
        })
        .sum()
}

fn main() {
    let code = load_input("./day3/input.txt");
    let ops = parse_input(&code);

    println!("Part 1: {}", part1(&ops));
    println!("Part 2: {}", part2(&ops));
}
