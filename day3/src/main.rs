
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::{vec, env};

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
    let file_path = args[1].clone();
    let lines: Vec<String> = read_lines_into_vector(&file_path).unwrap();

    let mut symbol_locations: Vec<Vec<i32>> = Vec::new();
    let mut possible_gears: Vec<Vec<usize>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        symbol_locations.push(Vec::new());
        possible_gears.push(Vec::new());

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() || c == '.' {
                continue;
            } else {
                symbol_locations[i].push(j as i32);

                if c == '*' {
                    possible_gears[i].push(j);
                }
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
        if location.len() == 0 {
            continue;
        }

        let aboveline: Vec<(usize, usize, &str)> = if line_num > 0 {
            let topline = lines.get(line_num - 1).unwrap();
            re.find_iter(topline)
                .map(|m| (m.start(), m.end(), m.as_str()))
                .collect()
        } else {
            Vec::new()
        };

        let line = lines.get(line_num).unwrap();
        let sameline: Vec<(usize, usize, &str)> = re
            .find_iter(line)
            .map(|m| (m.start(), m.end(), m.as_str()))
            .collect();
        let lowerline: Vec<(usize, usize, &str)> = if line_num < lines.len() - 1 {
            let bottomline = lines.get(line_num + 1).unwrap();

            re.find_iter(bottomline)
                .map(|m| (m.start(), m.end(), m.as_str()))
                .collect()
        } else {
            Vec::new()
        };

        let mut adj_nums = vec![];

        for maybe_gear in location{
            
        }

        for (start, stop, num) in sameline {
            if stop < line.len() && line.chars().nth(stop).unwrap() == '*' {
                adj_nums.push(num.parse::<u32>().unwrap());
            } else if start != 0 && line.chars().nth(start - 1).unwrap() == '*' {
                adj_nums.push(num.parse::<u32>().unwrap());
            }
        }

        for (start, stop, num) in aboveline {
            let left = if start == 0 { 0 } else { start - 1 };

            if location.iter().any(|x| x >= &left && x <= &stop) {
                adj_nums.push(num.parse::<u32>().unwrap());
            }
        }

        for (start, stop, num) in lowerline {
            let left = if start == 0 { 0 } else { start - 1 };
            let right = if stop < lines.get(line_num + 1).unwrap().len(){
                stop + 1
            }else{
                stop
            };


            println!("{} {} {} {:?}", left, right, num, location);
            if location.iter().any(|x| x >= &left && x  <= &stop) {
                adj_nums.push(num.parse::<u32>().unwrap());
            }
        }

        if adj_nums.len() == 2 {
            gear_ratios.push(adj_nums.iter().product::<u32>());
        }
        else if adj_nums.len() > 2{
            println!("{:?}", adj_nums);
            println!("too many nums!");
        }
    }

    // println!("SUM1 {:?}", nums.iter().sum::<u32>());
    println!("{:?}", gear_ratios.iter().sum::<u32>());
}
