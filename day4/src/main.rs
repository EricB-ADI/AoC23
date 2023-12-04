use std::collections::HashMap;
use regex::Regex;
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

    for line in &lines {
        let ginfo: Vec<&str> = line.split(":").collect();
        let card_num = ginfo
            .get(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap_or_default();


        let nums: Vec<&str> = ginfo.get(1).unwrap().split("|").collect();
        let winning: Vec<&str> = nums[0].split_whitespace().collect();
        let your_nums: Vec<&str> = nums[1].split_whitespace().collect();

        let mut game_worth = 0;
        for num in winning {
            if your_nums.contains(&num) {
                if game_worth == 0 {
                    game_worth = 1;
                } else {
                    game_worth *= 2;
                }
            }
        }
        worth += game_worth;
    }


    let mut card_copies = HashMap::new();

    for line in &lines {
        let ginfo: Vec<&str> = line.split(":").collect();
        let card_num = ginfo
            .get(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap().parse::<u32>().unwrap();


        let nums: Vec<&str> = ginfo.get(1).unwrap().split("|").collect();
        let winning: Vec<&str> = nums[0].split_whitespace().collect();
        let your_nums: Vec<&str> = nums[1].split_whitespace().collect();

        let mut winners = 0;
        for num in winning {
            if your_nums.contains(&num) {
                winners += 1;
                
                let card_to_copy = card_num + winners;

                let current_copies = match card_copies.get(&card_to_copy) {
                    Some(num) => *num,
                    None => 0
                };
                
                card_copies.insert(card_to_copy, current_copies + 1);

            }
        }
    
    }
    println!("{:?}", card_copies);
    println!("{}", worth);
}
