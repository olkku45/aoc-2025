use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_password() -> Result<(), std::io::Error> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut dial_pos = 50;
    
    for line in reader.lines() {
        let l = line?;
        let c = l.chars().nth(0).unwrap();
        
        let mut num = l.chars();
        num.next();

        let s = String::from(num.as_str());
        let parsed: Result<i32, _> = s.parse();
        
        if c == 'L' {            
            dial_pos -= parsed.unwrap();
        } else {
            dial_pos += parsed.unwrap();
        }

        dial_pos %= 100;
        if dial_pos == 0 { count += 1; }
    }

    println!("{}", count);
    Ok(())
}

fn main() {
    match get_password() {
        Ok(y) => y,
        Err(n) => println!("{}", n),
    }
}
