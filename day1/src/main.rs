use std::cmp::min;
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

        let cal_num: i32 = cal_str.parse().unwrap();

        cal_values.push(cal_num);
    }

    println!("SUM {}", cal_values.iter().sum::<i32>());

    //Part 2
    let mut num_lut = HashMap::new();
    num_lut.insert("one", "1");
    num_lut.insert("two", "2");
    num_lut.insert("three", "3");
    num_lut.insert("four", "4");
    num_lut.insert("five", "5");
    num_lut.insert("six", "6");
    num_lut.insert("seven", "7");
    num_lut.insert("eight", "8");
    num_lut.insert("nine", "9");
    num_lut.insert("1", "1");
    num_lut.insert("2", "2");
    num_lut.insert("3", "3");
    num_lut.insert("4", "4");
    num_lut.insert("5", "5");
    num_lut.insert("6", "6");
    num_lut.insert("7", "7");
    num_lut.insert("8", "8");
    num_lut.insert("9", "9");

    

    let mut cal_nums = vec![];

    for line in lines {
        let mut present_nums = HashMap::new();

        for (num, _) in &num_lut {
            if let Some(index) = line.find(num) {
             
             
                present_nums.insert(index, num_lut.get(num).unwrap());
            }
        }


        let mut min_key = 100;
        let mut max_key = 0;
        for(key, _) in &present_nums{
            if *key  < min_key{
                min_key = *key;
            }

            if *key > max_key{
                max_key = *key;
            }
        }   
        

        let min_num = present_nums.get(&min_key).unwrap();
        let max_num = present_nums.get(&max_key).unwrap();

        let num_str = format!("{min_num}{max_num}");
        cal_nums.push(num_str.parse::<i32>().unwrap());
        
    }
    let part2_sum = cal_nums.iter().sum::<i32>();
    println!("SUM2 {}", part2_sum);
}
