use std::{ env, fs };

mod day_one;
use day_one::day_one;
mod day_two;
use day_two::day_two;
mod day_three;
use day_three::day_three;
mod day_four;
use day_four::day_four;
mod day_five;
use day_five::day_five;
mod day_six;
use day_six::day_six;

fn main() -> Result<(), &'static str> {
    let mut args = env::args();

    // skip filename
    args.next();

    let day = match args.next() {
        Some(arg) => match arg.parse::<u8>() {
            Ok(val) => val,
            _ => return Err("The specified day needs to be a positive integer")
        },
        None => return Err("You didn't specify a day")
    };
    let test = match args.next() {
        Some(_) => ".test",
        _ => ""
    };
    let filename = format!("input-files/{}{}.txt", day, test);
    let input = match fs::read_to_string(filename) {
        Ok(content) => content,
        _ => return Err("Error while opening the input file")
    };

    match day {
        1 => day_one(&input),
        2 => day_two(&input),
        3 => day_three(&input),
        4 => day_four(&input),
        5 => day_five(&input),
        6 => day_six(&input),
        _ => return Err("No solution for this day found")
    }

    Ok(())
}