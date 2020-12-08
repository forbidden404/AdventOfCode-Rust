use std::path::Path;

pub fn main_2015_6() {
    let path = Path::new("src/_2015/inputs/day06.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

enum Move {
    TurnOn,
    TurnOff,
    Toggle,
}

fn process_one(input: &str) {
    let mut grid = [[false; 1000]; 1000];
    for line in input.lines() {
        let start_through = line
            .find(" through ")
            .expect("Line doesn't contain word through");
        let end_through = start_through + " through ".len();
        let movement: Move;
        if line.starts_with("turn on") {
            movement = Move::TurnOn;
        } else if line.starts_with("turn off") {
            movement = Move::TurnOff;
        } else {
            movement = Move::Toggle;
        }

        let from_str = match movement {
            Move::TurnOff => &line[9..start_through],
            Move::TurnOn => &line[8..start_through],
            Move::Toggle => &line[7..start_through],
        };

        let to_str = &line[end_through..];
        let mut from = from_str.split(",").map(|s| s.parse::<usize>().unwrap());
        let mut to = to_str.split(",").map(|s| s.parse::<usize>().unwrap());

        let from = (from.next().unwrap(), from.next().unwrap());
        let to = (to.next().unwrap(), to.next().unwrap());

        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                match movement {
                    Move::TurnOff => grid[x][y] = false,
                    Move::TurnOn => grid[x][y] = true,
                    Move::Toggle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }

    let mut lights = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] {
                lights += 1;
            }
        }
    }
    println!("{} are on.", lights);
}

fn process_two(input: &str) {
    let mut grid = [[0; 1000]; 1000];
    for line in input.lines() {
        let start_through = line
            .find(" through ")
            .expect("Line doesn't contain word through");
        let end_through = start_through + " through ".len();
        let movement: Move;
        if line.starts_with("turn on") {
            movement = Move::TurnOn;
        } else if line.starts_with("turn off") {
            movement = Move::TurnOff;
        } else {
            movement = Move::Toggle;
        }

        let from_str = match movement {
            Move::TurnOff => &line[9..start_through],
            Move::TurnOn => &line[8..start_through],
            Move::Toggle => &line[7..start_through],
        };

        let to_str = &line[end_through..];
        let mut from = from_str.split(",").map(|s| s.parse::<usize>().unwrap());
        let mut to = to_str.split(",").map(|s| s.parse::<usize>().unwrap());

        let from = (from.next().unwrap(), from.next().unwrap());
        let to = (to.next().unwrap(), to.next().unwrap());

        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                match movement {
                    Move::TurnOff => grid[x][y] = std::cmp::max(0, grid[x][y] - 1),
                    Move::TurnOn => grid[x][y] += 1,
                    Move::Toggle => grid[x][y] += 2,
                }
            }
        }
    }

    let mut brightness = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            brightness += grid[x][y];
        }
    }
    println!("The total brightness is {}.", brightness);
}
