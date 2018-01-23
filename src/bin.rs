extern crate ma_mapper;

use std::io;
use std::io::prelude::*;                                                           
use ma_mapper::board::Board;

fn main() {
    let mut input = String::new();

    print!("Enter Number of Tiles: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<usize>() {
                Ok(num_tiles) => Board::new(num_tiles, num_tiles / 2, num_tiles / 2).display(),
                Err(error) => eprintln!("Not a valid number: error: {}", error),
            };
        },
        Err(error) => eprintln!("Error getting input: {}", error),
    };
}
