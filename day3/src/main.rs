use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

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
    let file_path: &str = "input.txt";
    let lines: Vec<String> = read_lines_into_vector(file_path).unwrap();

    let mut symbol_location: Vec<Vec<i32>> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        symbol_location.push(Vec::new());

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() || c == '.' {
                continue;
            } else {
                symbol_location[i].push(j as i32);
            }
        }
    }

    let mut nums = vec![];
    let re = Regex::new(r"(\d+)").unwrap();
    for (line_num, line) in lines.iter().enumerate() {
        let numbers_with_indices: Vec<(i32, i32, &str)> = re
            .find_iter(line)
            .map(|m| (m.start() as i32, m.end() as i32, m.as_str()))
            .collect();

        for (start, stop, num) in numbers_with_indices {
            let leftbound = start - 1;
            let rightbound = stop; // rust iterator is one more than the last index
            let topbound: i32 = line_num as i32 - 1;
            let bottombound: i32 = line_num as i32 + 1;

            let num_parsed = num.parse::<u32>().unwrap();
            if symbol_location[line_num].contains(&leftbound)
                || symbol_location[line_num].contains(&rightbound)
            {
                nums.push(num_parsed);
                continue;
            }

            if topbound >= 0 {
                if symbol_location[topbound as usize]
                    .iter()
                    .any(|x: &i32| x >= &leftbound && x <= &rightbound)
                {
                    nums.push(num_parsed);
                    continue;
                }
            }

            if bottombound < lines.len() as i32 {
                if symbol_location[bottombound as usize]
                    .iter()
                    .any(|x: &i32| x >= &leftbound && x <= &rightbound)
                {
                    nums.push(num_parsed);
                }
            }
        }
    }
    
    println!("SUM1 {:?}", nums.iter().sum::<u32>());
}
