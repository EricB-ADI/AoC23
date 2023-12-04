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

fn get_card_matches(card_nums: &str, winning_nums: &str) -> u32 {
    let card_nums: Vec<&str> = card_nums.split_whitespace().collect();
    let winning_nums: Vec<&str> = winning_nums.split_whitespace().collect();

    let mut matches: u32 = 0;
    for num in card_nums {
        if winning_nums.contains(&num) {
            matches += 1;
        }
    }

    return matches;
}

fn get_num_cards(indices: &Vec<usize>, all_cards: &Vec<String>,) -> u32 {
    let mut num_cards: u32 = 0;

    for idx in indices {
        let card = &all_cards[*idx];
        let card: Vec<&str> = card.split(":").collect();
        let card_num: u32 = card[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let nums: Vec<&str> = card.get(1).unwrap().split("|").collect();

        let matches: u32 = get_card_matches(nums[0], nums[1]);
        num_cards += matches;

        let new_indices: Vec<usize> = (card_num as usize ..(card_num as usize +matches as usize)).collect();
        num_cards += get_num_cards(&new_indices, all_cards);
    }
    
    return num_cards
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines: Vec<String> = match args.get(1) {
        Some(fp) => read_lines_into_vector(&fp).unwrap(),
        None => panic!("Expected file path of input!"),
    };

    let initial_idx: Vec<usize> = (0..lines.len()).collect();

    println!("{}", lines.len() as u32 + get_num_cards(&initial_idx, &lines))

}