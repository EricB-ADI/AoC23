use core::num;
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

fn solve_pls(indicies: &Vec<usize>, all: &Vec<String>) ->u32
{
    
    if indicies.is_empty(){ return 0};

    let mut total = 0;
    for index in indicies{

        let card = all[*index].clone();

        let ginfo: Vec<&str> = card.split(":").collect();
            let card_num = ginfo
            .get(0)
            .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap().parse::<u32>().unwrap();
    
            let mut card_copies = vec![];
            let nums: Vec<&str> = ginfo.get(1).unwrap().split("|").collect();
            let winning: Vec<&str> = nums[0].split_whitespace().collect();
            let your_nums: Vec<&str> = nums[1].split_whitespace().collect();
            let mut winners:u32 = 0;
            for num in winning {
                if your_nums.contains(&num) {
                    winners += 1;
                    card_copies.push(winners as usize + card_num as usize - 1 as usize);
                
                }
            }
            total += winners;

                
            total += solve_pls(&card_copies,all);
            

    }
    return total;

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


    let init_idx: Vec<usize> = (0..lines.len()).collect();
    
    println!("{}", worth);
    println!("{}", lines.len() as u32 + solve_pls(&init_idx, &lines));

    
}
