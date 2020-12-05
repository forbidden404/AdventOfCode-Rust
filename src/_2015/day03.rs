use std::collections::HashMap;
use std::path::Path;

pub fn main_2015_3() {
    let path = Path::new("src/_2015/inputs/day03.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    let unique_houses = input
  		.chars()
  		.map(|c| match c {
          	'^' => (0, 1),
          	'>' => (1, 0),
          	'v' => (0, -1),
          	'<' => (-1, 0),
          	input => panic!("Unexpected input: `{}`", input),
        })
  		.scan((0, 0), |state, (x, y)| {
        	*state = (state.0 + x, state.1 + y);
          	Some(*state)
        })
  		.fold(
          	{
              let mut map = HashMap::new();
              map.insert((0, 0), 1);
              map
            },
          	|mut acc, (x, y)| {
            	acc.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
              	acc
            },
  		)
  		.len();
    println!("{} houses got at least one present.", unique_houses);
}

fn process_two(input: &str) {
    let mut is_robot = false;
    let unique_houses = input
  		.chars()
  		.map(|c| match c {
          	'^' => {                    
                let state = (0, 1, is_robot);
                is_robot = !is_robot;
                state
            }
          	'>' => {
                let state = (1, 0, is_robot);
                is_robot = !is_robot;
                state
            }
          	'v' => {
                let state = (0, -1, is_robot);
                is_robot = !is_robot;
                state
            }
          	'<' => {
                let state = (-1, 0, is_robot);
                is_robot = !is_robot;
                state
            }
          	input => panic!("Unexpected input: `{}`", input),
        })
  		.scan(((0, 0), (0, 0)), |state, (x, y, is_robot)| {
            if is_robot {
                *state = (state.0, (state.1.0 + x, state.1.1 + y));
            } else {
                *state = ((state.0.0 + x, state.0.1 + y), state.1);
            }
          	Some(*state)
        })
  		.fold(
          	{
              let mut map = HashMap::new();
              map.insert((0, 0), 1);
              map
            },
          	|mut acc, (x, y)| {
            	acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
            	acc.entry(y).and_modify(|e| *e += 1).or_insert(1);
              	acc
            },
  		)
  		.len();
    println!("{} houses got at least one present from Santa and its robot.", unique_houses);
}

