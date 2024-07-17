use std::collections::HashMap;

pub fn get_first_integer(s: &str) -> i32 {
    let mut res:i32 = 0;
    let mut char_index = 0;
    while char_index < s.len() {
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
    // println!("{:?}", spelled_out_ints);
    for ints in spelled_out_ints.iter() {
        let (index, value) = ints;
        if *index < char_index as i32 {
            char_index = *index as usize;
            res = *value;
        }
    } 
    // println!("{res}");
    res
}

pub fn get_last_integer(s: &str) -> i32 {
    let mut res: i32 = 0;
    let mut char_index: i32 = s.len() as i32 - 1;
    while char_index > -1 {
        match s.chars().nth(char_index as usize) {
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
    // println!("{:?}", spelled_out_ints);
    for ints in spelled_out_ints.iter() {
        let (index, value) = ints;
        if *index > char_index as i32 {
            char_index = *index;
            res = *value;
        }
    } 
    // println!("{res}");
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
    for (key, value) in hashmap {
        let mut start_pos = 0; 
        while let Some(pos) = s[start_pos..].find(&key) {
            res_vec.push(((pos+start_pos) as i32, value));
            start_pos = start_pos + pos +  key.len();
        }
    }
    res_vec
}