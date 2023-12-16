use std::fs;

#[allow(dead_code)]
fn part_one() {
    let input = fs::read_to_string("src/day_15/part_one_input.txt").unwrap();
    let initialization_sequence = input.as_str();
    let mut sum = 0;

    for step in initialization_sequence.split(',') {
        sum += hash(step);
    }

    println!("{sum}");
}

fn as_int(c: char) -> i32 {
    c as i32
}

fn hash(key: &str) -> i32 {
    let mut hash = 0;

    for char in key.chars().map(as_int) {
        hash += char;
        hash *= 17;
        hash %= 256;
    }

    println!("key {{{key} hash {hash}");
    hash
}

#[cfg(test)]
mod tests {
    use crate::day_15::part_one::{hash, part_one};

    #[test]
    fn test_hash() {
        hash("HASH");
    }

    #[test]
    fn test_part_one() {
        part_one();
    }
}
