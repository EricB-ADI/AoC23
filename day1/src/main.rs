use itertools::{MinMaxResult, Itertools};
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

fn main() {
    let file_path = "input.txt"; // Replace with the actual file path

    let lines = read_lines_into_vector(file_path).unwrap();
    let mut cal_values = Vec::new();

    for line in &lines {
        let first_index = line.find(char::is_numeric).unwrap();

        let last_index = match line.chars().rev().position(char::is_numeric) {
            Some(n) => line.len() - n - 1,
            None => {
                println!("No numeric characters found.");
                return;
            }
        };

        let char1 = line.chars().nth(first_index).unwrap();
        let char2 = line.chars().nth(last_index).unwrap();

        let cal_str = format!("{}{}", char1, char2);

        let cal_num: u32 = cal_str.parse().unwrap();

        cal_values.push(cal_num);
    }

    println!("SUM {}", cal_values.iter().sum::<u32>());

    //Part 2
    let num_lut = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ]);

    let mut cal_nums = vec![];

    for line in lines {
        let mut present_nums = HashMap::new();

        for (num, _) in &num_lut {
            if let Some(index) = line.find(num) {
                present_nums.insert(index, num_lut.get(num).unwrap());
            }
            if let Some(index) = line.rfind(num) {
                present_nums.insert(index, num_lut.get(num).unwrap());
            }
        }

        let minmax = present_nums.keys().into_iter().minmax();
        
        let (min_key, max_key) = match minmax{
            MinMaxResult::MinMax(min_val, max_val) => (min_val,max_val),
            MinMaxResult::OneElement(elem) => (elem, elem),
            MinMaxResult::NoElements => panic!("Gotta be something")
        };


        let min_num = present_nums.get(&min_key).unwrap();
        let max_num = present_nums.get(&max_key).unwrap();

        let num_str = format!("{min_num}{max_num}");

        cal_nums.push(num_str.parse::<u32>().unwrap());
    }
    
    println!("SUM2 {}", cal_nums.iter().sum::<u32>());
}
