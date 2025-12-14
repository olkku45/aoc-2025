fn split_recursively(num: i64, chunk_size: usize) -> bool {
    let length = num.to_string().len();

    if length == 1 { return false; }

    if length % chunk_size == 0 {
        let slice = num.to_string().into_bytes();
        let vec_of_slices: Vec<&[u8]> = slice.chunks(chunk_size).collect();

        if vec_of_slices.iter().all(|item| item == &vec_of_slices[0]) {
            return true;
        }
    }

    if chunk_size > 1 {
        return split_recursively(num, chunk_size - 1);
    }
    return false;
}

fn do_thing() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let split_by_comma: Vec<&str> = file.split(',').collect();

    let mut counter: i64 = 0;
    
    for range in split_by_comma {
        let pair: Vec<&str> = range.split('-').collect();
        let clean1 = pair[0].replace("\n", "");
        let clean2 = pair[1].replace("\n", "");

        let low: i64 = clean1.parse().unwrap();
        let high: i64 = clean2.parse().unwrap();

        for n in low..=high {
            if split_recursively(n, n.to_string().len() / 2) {
                counter += n;
            }
        }
    }

    println!("{}", counter);
}

fn main() {
    do_thing();
}
