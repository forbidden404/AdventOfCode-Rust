use std::path::Path;

pub fn main_2015_4() {
    let path = Path::new("src/_2015/inputs/day04.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process_one(&input);
    process_two(&input);
}

fn process_one(input: &str) {
    let mut num = 1;
    loop {
        let mut data = input.to_string();
        data.push_str(num.to_string().as_str());
        let digest = format!("{:?}", md5::compute(data));            
        if &digest[..5] == "00000" {
            println!("Hash is {}.", digest);
            break;
        }

        num += 1;
    }

    println!("The lowest number to create a md5 hash with 5 leading zeroes is {}.", num);
}

fn process_two(input: &str) {
    let mut num = 1;
    loop {
        let mut data = input.to_string();
        data.push_str(num.to_string().as_str());
        let digest = format!("{:?}", md5::compute(data));            
        if &digest[..6] == "000000" {
            println!("Hash is {}.", digest);
            break;
        }

        num += 1;
    }

    println!("The lowest number to create a md5 hash with 6 leading zeroes is {}.", num);
}

