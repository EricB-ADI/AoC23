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
    let file_path: &str = "input.txt";

    let lines: Vec<String> = read_lines_into_vector(file_path).unwrap();

    let num_cubes: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut valid_games: Vec<u32> = Vec::new();
    let mut power_cubes: Vec<u32> = Vec::new();

    for line in lines {
        let game_info: Vec<&str> = line.split(":").collect();

        let sets: Vec<&str> = game_info[1].split(";").collect();
        let game_id: u32 = game_info[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut valid_game: bool = true;

        for set in &sets {
            let colors = parse_set(&set);

            valid_game = num_cubes
                .iter()
                .all(|(key, value)| colors.get(*key).map_or(true, |&c| c <= *value));

            if !valid_game {
                break;
            }
        }

        let mut cubes_needed: HashMap<String, u32> = HashMap::new();

        for set in &sets {
            let colors: HashMap<String, u32> = parse_set(&set);

            for (color, value) in colors {
                if let Some(cubes) = cubes_needed.get(&color) {
                    if value >= *cubes {
                        cubes_needed.insert(color, value);
                    }
                } else {
                    cubes_needed.insert(color, value);
                }
            }
        }

        power_cubes.push(cubes_needed.values().product());

        if valid_game {
            valid_games.push(game_id);
        }
    }

    // println!("{:?}", valid_games);
    println!("Sum {}", valid_games.iter().sum::<u32>());
    // println!("{:?}", power_cubes);
    println!("Power cubes {}", power_cubes.iter().sum::<u32>());
}
