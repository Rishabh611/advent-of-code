use std::fs;
use std::error::Error;
use day1::get_first_integer;
use day1::get_last_integer;
fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;
    let mut res = 0;
    for line in contents.lines() {
        let temp;
        temp = get_first_integer(line) * 10 + get_last_integer(line);
        println!("{} {} {}" , temp, get_first_integer(line), get_last_integer(line) );
        res = res +temp;
    }
    println!("{res}");
    Ok(())
}
