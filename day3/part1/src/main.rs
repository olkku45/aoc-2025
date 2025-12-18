use std::fs::File;
use std::io::{prelude::*, BufReader};

fn biggest_joltage(line: String) -> u32 {
    let mut arr: Vec<u32> = Vec::new();
    
    let bytes = line.as_bytes();
    for i in 0..bytes.len() {
        let ch1 = bytes[i];
        for j in i + 1..bytes.len() {
            let ch2 = bytes[j];

            // ascii value to digit
            let corr_ch1 = ch1 - 48;
            let corr_ch2 = ch2 - 48;

            let as_str1 = corr_ch1.to_string();
            let as_str2 = corr_ch2.to_string();
            let temp = as_str2.as_str();
            let combined = as_str1 + temp;

            let num: u32 = combined.parse().unwrap();
            arr.push(num);
        }
    }

    let val: u32 = *arr.iter().max().unwrap();
    return val;
}

fn get_total_joltage() -> Result<(), std::io::Error> {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_joltage: u32 = 0;

    for line in reader.lines() {
        let l = line?;
        let max = biggest_joltage(l);
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
