use std::path::Path;

pub fn main_2015_1() {
    let path = Path::new("src/_2015/inputs/day01.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    let floor = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            input => panic!("Unexpected input `{}`", input),
        })
        .fold(0, |acc, x| {
            acc + x
        });
    println!("The floor is {}", floor);
}

fn process_two(input: &str) {
    let floors_until_basement: Vec<_> = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            input => panic!("Unexpected input `{}`", input),
        })
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .take_while(|x| *x >= 0)
        .collect();
    println!("The position of the character that caused Santa to enter the basement is {}", floors_until_basement.len() + 1);
}