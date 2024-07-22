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


pub fn get_power(contents : &str) -> i32 {
    let mut power: i32 = 0;

    for game in contents.lines() {
        let draw_str: Vec<&str> = game.split(":").collect();
        let mut max_count_of_each_color: HashMap<&str, i32> = HashMap::new();

        max_count_of_each_color.insert("red", 0);
        max_count_of_each_color.insert("green", 0);
        max_count_of_each_color.insert("blue", 0);

        let draws = get_games_vector(draw_str.last().unwrap());

        for draw in draws {
            let color_str_in_draw: Vec<&str> = draw.split(",").collect();

            for color_draw in color_str_in_draw {
                let color_draw = color_draw.trim();
                let color_parts: Vec<&str> = color_draw.split(" ").collect();

                let color = *color_parts.last().unwrap();

                let color_count: i32 = color_parts.first().unwrap().parse().unwrap();

                if color_count > *max_count_of_each_color.get(color).unwrap() {
                    max_count_of_each_color.insert(color, color_count);
                }
            }
        }
        let mut game_power = 1;

        for value in max_count_of_each_color.values() {
            game_power *= value;
        }

        power += game_power;
    }

    power
}
