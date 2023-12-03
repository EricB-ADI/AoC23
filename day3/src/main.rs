use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, vec};

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
    let file_path = match args.get(1){
        Some(fp) => fp,
        None => panic!("Expected file path of input!")
    };
    let lines: Vec<String> = read_lines_into_vector(&file_path).unwrap();

    let mut symbol_locations: Vec<Vec<i32>> = Vec::new();
    let mut possible_gears: Vec<Vec<usize>> = Vec::new();
    let mut parsed_lines = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    for (i, line) in lines.iter().enumerate() {
        symbol_locations.push(Vec::new());
        possible_gears.push(Vec::new());

        for (j, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                symbol_locations[i].push(j as i32);
                if c == '*' {
                    possible_gears[i].push(j);
                }
            }
        }
        let parsed: Vec<(usize, usize, u32)> = re
            .find_iter(line)
            .map(|m| (m.start(), m.end(), m.as_str().parse().unwrap()))
            .collect();
        parsed_lines.push(parsed);
    }

    let mut nums = vec![];

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

            if symbol_locations[line_num].contains(&leftbound)
                || symbol_locations[line_num].contains(&rightbound)
            {
                nums.push(num_parsed);
                continue;
            }

            if topbound >= 0 {
                if symbol_locations[topbound as usize]
                    .iter()
                    .any(|x: &i32| x >= &leftbound && x <= &rightbound)
                {
                    nums.push(num_parsed);
                    continue;
                }
            }

            if bottombound < lines.len() as i32 {
                if symbol_locations[bottombound as usize]
                    .iter()
                    .any(|x: &i32| x >= &leftbound && x <= &rightbound)
                {
                    nums.push(num_parsed);
                }
            }
        }
    }

    let mut gear_ratios = vec![];

    for (line_num, location) in possible_gears.iter().enumerate() {
        if location.is_empty() {
            continue;
        }

        let sameline: Vec<(usize, usize, u32)> =
            parsed_lines.get(line_num).cloned().unwrap_or_default();
        let aboveline: Vec<(usize, usize, u32)> = parsed_lines
            .get(line_num.wrapping_sub(1))
            .cloned()
            .unwrap_or_default();
        let lowerline: Vec<(usize, usize, u32)> =
            parsed_lines.get(line_num + 1).cloned().unwrap_or_default();

        for maybe_gear in location {
            let mut adj_nums = vec![];

            for (start, stop, num) in &sameline {
                if *stop == *maybe_gear || maybe_gear + 1 == *start {
                    adj_nums.push(*num);
                }
            }
            for (start, stop, num) in &aboveline {
                if *start <= *maybe_gear + 1 && *maybe_gear <= *stop {
                    adj_nums.push(*num);
                }
            }
            for (start, stop, num) in &lowerline {
                let left = if *start == 0 { 0 } else { *start - 1 };
                if left <= *maybe_gear && *maybe_gear <= *stop {
                    adj_nums.push(*num);
                }
            }

            if adj_nums.len() == 2 {
                gear_ratios.push(adj_nums.iter().product::<u32>());
            }
        }
    }

    println!("SUM1 {:?}", nums.iter().sum::<u32>());
    println!("GEAR SUM {:?}", gear_ratios.iter().sum::<u32>());
}
