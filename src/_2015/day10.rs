use std::path::Path;

pub fn main_2015_10() {
    let path = Path::new("src/_2015/inputs/day10.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    println!("Length of new string is {}", process(input, 40));
}

fn process_two(input: &str) {
    println!("Length of new string is {}", process(input, 50));
}

fn process(input: &str, times: usize) -> usize {
    let mut new_str = String::from(input);
    for _ in 0..times {
        let mut tmp_str = String::new();
        let mut prev_char = None;
        let mut counter = 1;
        for (i, c) in new_str.chars().enumerate() {
            if let Some(prev_char) = prev_char {
                if prev_char == c { 
                    counter += 1;
                } else {
                    tmp_str.push_str(&format!("{}{}", counter, prev_char));
                    counter = 1;
                }
            } else {
                counter = 1;
            }

            if i == new_str.len() - 1 {
                tmp_str.push_str(&format!("{}{}", counter, c));
            }

            prev_char = Some(c);
        }
        new_str = String::from(tmp_str);
    }
    new_str.len()
}
