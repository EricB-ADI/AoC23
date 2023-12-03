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
fn part1(lines: &Vec<String>, parsed_lines: &Vec<Vec<(usize, usize, u32)>>, symbol_locations:&Vec<Vec<usize>>) -> u32{

    let mut nums = vec![];

    for (line_num, _) in lines.iter().enumerate() {
        for (start, stop, num) in &parsed_lines[line_num] {
            let leftbound = start.wrapping_sub(1);
            let rightbound = *stop; // rust iterator is one more than the last index
            let topbound = line_num.wrapping_sub(1);
            let bottombound= line_num  + 1;
            let num_parsed = *num;

            if symbol_locations[line_num]
                .iter()
                .any(|loc| *loc == leftbound || *loc == rightbound)
            {
                nums.push(num_parsed);
                continue;
            }
            
            if topbound < lines.len()
                && symbol_locations[topbound]
                    .iter()
                    .any(|x| *x >= leftbound && *x <= rightbound)
            {
                nums.push(num_parsed);
                continue;
            }

            if bottombound < lines.len()
                && symbol_locations[bottombound as usize]
                    .iter()
                    .any(|x| *x >= leftbound && *x <= rightbound)
            {
                nums.push(num_parsed);
            }
        }
    }


    return  nums.iter().sum();
}

fn part2(possible_gears:Vec<Vec<usize>>, parsed_lines: &Vec<Vec<(usize, usize, u32)>>) -> u32{

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

    return gear_ratios.iter().sum();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let lines: Vec<String> = match args.get(1) {
        Some(fp) => read_lines_into_vector(&fp).unwrap(),
        None => panic!("Expected file path of input!"),
    };
    

    let mut symbol_locations: Vec<Vec<usize>> = Vec::new();
    let mut possible_gears: Vec<Vec<usize>> = Vec::new();
    let mut parsed_lines: Vec<Vec<(usize, usize, u32)>> = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    for (i, line) in lines.iter().enumerate() {
        symbol_locations.push(Vec::new());
        possible_gears.push(Vec::new());

        for (j, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                symbol_locations[i].push(j);
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

    println!("SUM1 {:?}", part1(&lines, &parsed_lines, &symbol_locations));
    println!("GEAR SUM {:?}", part2(possible_gears, &parsed_lines));
}
