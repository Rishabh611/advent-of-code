use std::collections::HashMap;

pub fn get_first_integer(s: &str) -> i32 {
    let mut res:i32;
    let mut char_index = 0;
    loop {
        if char_index > s.len(){
            break;
        }
        match s.chars().nth(char_index) {
            Some(c) => {
                if c.is_digit(10) {
                    res = c.to_digit(10).unwrap_or(0) as i32;
                    break;
                }
            },
            None => (),
        }
        char_index += 1;
    }
    let spelled_out_ints = get_spelled_out_integer(s);
    if get_spelled_out_integer(s).len() > 1 {
        let (index, value) = spelled_out_ints.first().unwrap();
        if *index < char_index as i32 {
            res = *value;
        }
    } 
    res
}

pub fn get_last_integer(s: &str) -> i32 {
    let mut res: i32;
    let mut char_index: usize = s.len() as usize - 1;
    loop {
        if char_index < 0 {
            break;
        }
        match s.chars().nth(char_index) {
            Some(c) => {
                if c.is_digit(10) {
                    res = c.to_digit(10).unwrap_or(0) as i32;
                    break;
                }
            },
            None => (),
        }
        char_index -= 1;
    }
    let spelled_out_ints = get_spelled_out_integer(s);
    if spelled_out_ints.len() > 0 {
        let (index, value) = spelled_out_ints.last().unwrap();
        if  *index > char_index as i32 {
            res = *value;
        }
    }
    
    res
}

pub fn get_spelled_out_integer(s: &str) -> Vec<(i32, i32)> {
    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("one"), 1);
    hashmap.insert(String::from("two"), 2);
    hashmap.insert(String::from("three"), 3);
    hashmap.insert(String::from("four"), 4);
    hashmap.insert(String::from("five"), 5);
    hashmap.insert(String::from("six"), 6);
    hashmap.insert(String::from("seven"), 7);
    hashmap.insert(String::from("eight"), 8);
    hashmap.insert(String::from("nine"), 9);
    hashmap.insert(String::from("zero"), 0);

    let mut res_vec: Vec<(i32, i32)> = vec![];
    let mut start_pos = 0;
    for (key, value) in hashmap {
        while let Some(pos) = s[start_pos..].find(&key) {
            res_vec.push(((pos+start_pos) as i32, value));
            start_pos = start_pos + pos +  key.len();
        }
    }
    res_vec
}