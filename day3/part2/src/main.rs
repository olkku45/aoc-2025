use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::BTreeMap;

fn biggest_joltage(line: &String) -> u64 {
    let mut line_map: BTreeMap<usize, char> = BTreeMap::new();
    let mut start_index = 0;

    loop {
        if line_map.len() == 12 { break; }
        let mut curr_map: BTreeMap<usize, char> = BTreeMap::new();
        for i in start_index..=line.len() - 12 + line_map.len() {
            curr_map.insert(i, line.chars().nth(i).unwrap());
        }

        let temp = curr_map.clone();
        let max = temp.values().max().unwrap();
        
        if let Some((&pos, &digit)) = curr_map.iter().find(|(_, d)| **d == *max) {
            line_map.insert(pos, digit);
            start_index = pos + 1;
        }
    }

    let as_str: String = line_map.values().collect();
    let as_num: u64 = as_str.parse().unwrap();
    return as_num;
}

fn get_total_joltage() -> Result<(), std::io::Error> {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_joltage: u64 = 0;

    for line in reader.lines() {
        let l = line?;
        let max = biggest_joltage(&l);
        total_joltage += max;
    }

    println!("{}", total_joltage);

    Ok(())
}

fn main() {
    match get_total_joltage() {
        Ok(y) => y,
        Err(n) => println!("{}", n),
    };
}
