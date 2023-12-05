use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Map;
#[derive(Debug)]
struct Mapping {
    
    dest_start: u32,
    source_start: u32,
    range_length: u32,
}

impl Mapping {
    fn map(&self, source: u32) -> u32 {
        if source >= self.source_start && source < self.source_start + self.range_length {
            return self.dest_start + (source - self.source_start);
        }
        source
    }
    fn from(dest_start:u32, source_start:u32, range_length:u32) -> Mapping {

        Mapping {dest_start, source_start, range_length }
    }
}

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
    let seeds_planted:Vec<&str> = lines[0].split(":").collect();
    let seeds_planted: Vec<u32> = seeds_planted.get(1).unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    println!("{:?}", seeds_planted);
    let mut planting_maps :HashMap<&str, Vec<Mapping>>= HashMap::new();


    let mut current_map = String::new();
    
    for (i, line) in lines.iter().enumerate(){
        if i == 0 || line.is_empty(){
            continue;
        }
        if line.contains(":")
        {
            current_map = String::from(line);
            continue;
        }

        if let Some(&maps) = planting_maps.get(&current_map as &str)
        {
            let split :Vec<u32>= line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            maps.push(split);
        }
        else
        {

        }

        
        

            Some(map) => {
             let split :Vec<u32>= line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            map.copied().push(split);
            },
            None => {planting_maps.insert(&current_map, Vec::new());},
        
    }



    println!("{:?}", planting_maps);
}

