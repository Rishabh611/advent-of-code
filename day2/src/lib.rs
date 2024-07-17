use std::collections::HashMap;

pub fn possible_games(contents : &str) -> i32 {
    let mut possible_game_count: i32 = 0;

    let mut elf_condition: HashMap<&str, i32> = HashMap::new();

    elf_condition.insert("red", 12);
    elf_condition.insert("green", 13);
    elf_condition.insert("blue", 14);

    for line in contents.lines() {
        let game_str: Vec<&str> = line.split(":").collect();
        let game_id: i32 = get_game_id(game_str.first().unwrap());
        let game_content = get_games_vector(game_str.last().unwrap());
        let mut flag:bool = true;
        for game in game_content {
            let color_str: Vec<&str> = game.split(",").collect();
            for color in color_str {
                let color = color.trim();
                let color_parts: Vec<&str> = color.split(" ").collect();
                let color = *color_parts.last().unwrap();
                let color_count: i32 = color_parts.first().unwrap().parse().unwrap();
                if color_count > *elf_condition.get(color).unwrap() {
                    flag = false;
                }
            }            
        }
        if flag {
            possible_game_count += game_id;
        }
    }
    return possible_game_count;
}

fn get_game_id(game_str: &str) -> i32 {
    let game_str_parts : Vec<&str> = game_str.split(" ").collect();
    game_str_parts.last().unwrap().parse().unwrap()
}

fn get_games_vector(game_str : &str) -> Vec<&str> {
    let games_vec: Vec<&str> = game_str.split(";").collect();
    games_vec
}