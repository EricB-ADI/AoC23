use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


fn read_lines_into_vector(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn parse_set(set: &str) -> HashMap<String, u32> {
    let mut colors = HashMap::new();

    let sinfo: Vec<&str> = set.split_whitespace().collect();

    for i in 0..sinfo.len() {
        let elem = sinfo.get(i).unwrap();

        if let Ok(number) = elem.parse::<u32>() {
            let color = sinfo.get(i + 1).unwrap().replace(",", "");

            colors.insert(color, number);
        }
    }

    return colors;
}
fn main() {
    let file_path = "input.txt";

    let lines = read_lines_into_vector(file_path).unwrap();

    let num_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut valid_games = vec![];

    for line in lines {
        let game_info: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = game_info[1].split(";").collect();
        let game_id: Vec<&str> = game_info[0].split_whitespace().collect();
        let game_id = game_id[1].parse::<u32>().unwrap();

        let mut valid_game = true;
        for set in sets {
            let colors = parse_set(&set);


            for (key, value) in &num_cubes {
                let mut valid_set = true;
                if colors.contains_key(*key) {
                    valid_set = colors.get(*key).unwrap() <= value;
                }

                if !valid_set {
                    valid_game = false;
                    break;
                }
            }

            if !valid_game {
                break;
            }
        }
        if valid_game {
            valid_games.push(game_id);
        }
    }

    println!("{:?}", valid_games);
    println!("{}", valid_games.iter().sum::<u32>());
}
