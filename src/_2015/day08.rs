use std::path::Path;

pub fn main_2015_8() {
    let path = Path::new("src/_2015/inputs/day08.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    let diff = input
        .lines()
        .fold(0, |acc, line| {
            let mut prev_char = None;
            acc + line
                .chars()
                .fold(0, |acc, c| {
                    if prev_char == Some('\\') {
                        prev_char = Some(c);
                        if c == '\\' {
                            prev_char = None;
                            acc + 1
                        } else if c == 'x' {
                            acc + 3
                        } else {
                            acc + 1
                        }
                    } else if c == '\"' {
                        prev_char = Some(c);
                        acc + 1
                    } else {
                        prev_char = Some(c);
                        acc
                    }
                })
        });

    println!("The difference is {}", diff);
}

fn process_two(input: &str) {
    let diff = input
        .lines()
        .fold(0, |acc, line| {
            let mut prev_char = None;
            acc + line
                .chars()
                .fold(0, |acc, c| {
                    if prev_char == Some('\\') {
                        prev_char = Some(c);
                        if c == '\\' {
                            prev_char = None;
                            acc + 2
                        } else if c == 'x' {
                            acc + 1
                        } else {
                            acc + 2
                        }
                    } else if c == '\"' {
                        prev_char = Some(c);
                        acc + 2
                    } else {
                        prev_char = Some(c);
                        acc
                    }
                })
        });

    println!("The difference is {}", diff);
}