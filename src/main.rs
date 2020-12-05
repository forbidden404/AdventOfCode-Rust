#[cfg(feature = "2015")]
mod _2015;
#[cfg(feature = "2015")]
use _2015::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the year and day, e.g. `2015_1` for 2015/day01");
        std::process::exit(1);
    }

    let date_id = &args[1];
    match date_id.as_str() {
        #[cfg(feature = "2015")] "2015_1" => main_2015_1(),
        #[cfg(feature = "2015")] "2015_2" => main_2015_2(),
        #[cfg(feature = "2015")] "2015_3" => main_2015_3(),
        _ => println!("Day isn't completed yet.")
    }
}
