pub mod exos;
pub mod file;

use std::env;

use exos::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough args");
    } else {
        match (args[1].parse::<u8>(), args[2].parse::<u8>()) {
            (Ok(exo), Ok(part)) => match (exo, part) {
                (1, 1 | 2) => e01::run(part),
                (2, 1 | 2) => e02::run(part),
                _ => println!("This exercise does not exist"),
            },
            _ => println!("Error parsing args"),
        }
    }
}
