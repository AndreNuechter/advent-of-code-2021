use std::{ env, fs };

mod day_one;
use day_one::day_one;

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

    let input = match fs::read_to_string(format!("input-files/{}.txt", day)) {
        Ok(content) => content,
        _ => return Err("Error while opening the input file")
    };

    match day {
        1 => day_one(&input),
        _ => return Err("No solution for this day found")
    }

    Ok(())
}