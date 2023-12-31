use std::collections::HashSet;
use std::env;
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
fn main() {
    let args: Vec<String> = env::args().collect();

    let lines: Vec<String> = match args.get(1) {
        Some(fp) => read_lines_into_vector(&fp).unwrap(),
        None => panic!("Expected file path of input!"),
    };

    let mut worth = 0;

    let mut times_played = vec![1; lines.len()];

    for (game_num, line) in lines.iter().enumerate() {
        let ginfo: Vec<&str> = line.split(":").collect();
        let nums: Vec<&str> = ginfo.get(1).unwrap().split("|").collect();
        let winning_nums: HashSet<u32> = nums[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let your_nums: HashSet<u32> = nums[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut game_worth = 0;
        let mut total_matches = 0;

        for num in winning_nums {
            if your_nums.contains(&num) {
                total_matches += 1;
                if game_worth == 0 {
                    game_worth = 1;
                } else {
                    game_worth *= 2;
                }
            }
        }
        worth += game_worth;

        for w in 0..total_matches {
            times_played[game_num + w + 1] += times_played[game_num];
        }
    }

    println!("{}", worth);
    println!("{}", times_played.iter().sum::<i32>());
}
