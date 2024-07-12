pub fn get_first_integer(s: &str) -> i32 {
    let mut res:i32 = 0;
    for c in s.chars() {
        if c.is_digit(10) {
            res = c.to_digit(10).expect("Not valid int") as i32;
            break;
        }
    }
    res
}

pub fn get_last_integer(s: &str) -> i32 {
    let res: i32;
    let mut i: i32 = s.len() as i32 - 1;
    loop {
        if s.chars().nth(i as usize).unwrap_or('a').is_digit(10) {
            res = s.chars().nth(i as usize).unwrap_or('a').to_digit(10).expect("not valid int") as i32;
            return  res;
        }
        i-=1;
    }
}