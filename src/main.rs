use std::env;
use std::str::FromStr;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;

fn main() {
    let args: Vec<String> = env::args().collect();
    match usize::from_str(args
        .get(1)
        .expect("You have to provide a number to run a solution for given day!")
    ).expect("Provided argument couldn't be parsed as an unsigned number") {
        1 => day_1::main(),
        2 => day_2::main(),
        3 => day_3::main(),
        4 => day_4::main(),
        5 => day_5::main(),
        6 => day_6::main(),
        7 => day_7::main(),
        8 => day_8::main(),
        9 => day_9::main(),
        10 => day_11::main(),
        11 => day_11::main(),
        12 => day_12::main(),
        _ => panic!("No solution found for given day number!")
    }
}
