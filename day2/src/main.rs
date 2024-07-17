use std::{fs, io::Read};
use day2::possible_games;
fn main() {
    let mut input = fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    println!("{}", possible_games(&contents))
}
