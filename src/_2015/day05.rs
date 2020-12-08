use std::collections::HashSet;
use std::path::Path;

pub fn main_2015_5() {
    let path = Path::new("src/_2015/inputs/day05.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    let vowels = {
        let mut set = HashSet::new();
        set.insert('a');
        set.insert('e');
        set.insert('i');
        set.insert('o');
        set.insert('u');
        set
    };

    let mut nice_strings = 0;

    for line in input.lines() {
        let mut num_vowels = 0;

        let mut has_double_letter = false;
        let mut has_bad_string = false;
        let mut prev_char: Option<char> = None;
        for c in line.chars() {
            if vowels.contains(&c) {
                num_vowels += 1
            }

            if let Some(prev_char) = prev_char {
                if prev_char == c {
                    has_double_letter = true;
                }

                match (prev_char, c) {
                    ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => has_bad_string = true,
                    _ => {}
                }
            }

            prev_char = Some(c);
        }
        if num_vowels > 2 && has_double_letter && !has_bad_string {
            nice_strings += 1
        }
    }

    println!("There are {} nice strings.", nice_strings);
}

fn process_two(input: &str) {
    let mut nice_strings = 0;

    for line in input.lines() {
        if line.len() < 4 {
            continue;
        }

        let mut repeated_skipping_one = false;
        let mut double_substring = false;

        let mut start_index: usize = 0;
        let mut end_index: usize = 1;
        loop {
            let pivot = &line[start_index..end_index + 1];

            let mut exp_start_index = end_index + 1;
            let mut exp_end_index = exp_start_index + 1;
            if exp_start_index < line.len() {
                let fst_char = &line[start_index..start_index + 1];
                let snd_char = &line[exp_start_index..exp_start_index + 1];
                if fst_char == snd_char {
                    repeated_skipping_one = true;
                }

                while exp_end_index < line.len() {
                    if pivot == &line[exp_start_index..exp_end_index + 1] {
                        double_substring = true;
                        break;
                    }

                    exp_start_index += 1;
                    exp_end_index += 1;
                }

                if repeated_skipping_one && double_substring {
                    break;
                }
            }

            start_index += 1;
            end_index += 1;
            if end_index >= line.len() {
                break;
            }
        }

        if repeated_skipping_one && double_substring {
            nice_strings += 1
        }
    }

    println!("There are {} nice strings.", nice_strings);
}
