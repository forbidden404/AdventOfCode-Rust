use std::path::Path;

pub fn main_2015_2() {
    let path = Path::new("src/_2015/inputs/day02.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    let sqft = input.lines().fold(0, |acc, s| {
        let v = s
            .split("x")
            .map(|s| s.parse::<i32>().expect("Failed to parse number"))
            .collect::<Vec<i32>>();
        let lw = v[0] * v[1];
        let wh = v[1] * v[2];
        let hl = v[2] * v[0];
        let small = std::cmp::min(lw, std::cmp::min(wh, hl));
        acc + 2 * wh + 2 * lw + 2 * hl + small
    });
    println!("They should order {} square feet of wrapping paper.", sqft);
}

fn process_two(input: &str) {
    let feet = input.lines().fold(0, |acc, s| {
        let mut v = s
            .split("x")
            .map(|s| s.parse::<i32>().expect("Failed to parse number"))
            .collect::<Vec<i32>>();
        v.sort();

        acc + v[0] * v[1] * v[2] + 2 * v[0] + 2 * v[1]
    });
    println!("They should order {} feet of ribbon.", feet);
}
