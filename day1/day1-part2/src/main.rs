use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_password() -> Result<(), std::io::Error> {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut counter = 0;
    let mut dial_pos = 50;
    
    for line in reader.lines() {
        let l = line?;
        let c = l.chars().nth(0).unwrap();
        
        let mut num = l.chars();
        num.next();

        let s = String::from(num.as_str());
        let parsed: Result<i32, _> = s.parse();
        let mut parsed = parsed.unwrap();

        if c == 'L' {
            while parsed > 0 {
                dial_pos -= 1;
                if dial_pos % 100 == 0 { counter += 1; }
                parsed -= 1;
            }
        } else {
            while parsed > 0 {
                dial_pos += 1;
                if dial_pos % 100 == 0 { counter += 1; }
                parsed -= 1;
            }
        }

        println!("{}", dial_pos);
    }

    println!("{}", counter);
    Ok(())
}

fn main() {
    match get_password() {
        Ok(y) => y,
        Err(n) => println!("{}", n),
    }
}
