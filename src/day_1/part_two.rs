use std::fs;

use super::trie::Trie;

#[allow(dead_code)]
fn part_two() {
    let input = fs::read_to_string("src/day_1/input.txt");

    match input {
        Ok(value) => with_trie_look_up(&value),
        Err(e) => println!("Error: {e}"),
    }
}

fn with_trie_look_up(input: &str) {
    let mut sum = 0;
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let reversed_numbers = get_reversed_numbers(&numbers);
    let trie = build_trie::<&str>(&numbers);
    let reversed_trie = build_trie::<String>(reversed_numbers.as_slice());

    for line in input.lines() {
        let mut calibration = 0;

        // left look up
        for (i, char) in line.chars().enumerate() {
            let string = &line[i..];
            if let Some(number) = trie.find_first_largest(string) {
                calibration += number * 10;
                break;
            };

            if char.is_numeric() {
                let number = char.to_digit(10).unwrap();
                calibration += (number * 10) as i32;
                break;
            }
        }

        // right look up
        for (i, char) in line.chars().rev().enumerate() {
            if let Some(number) = reversed_trie.find_first_largest(
                line[..line.len() - i]
                    .chars()
                    .rev()
                    .collect::<String>()
                    .as_str(),
            ) {
                calibration += number;
                break;
            };

            if char.is_numeric() {
                let number = char.to_digit(10).unwrap();
                calibration += number as i32;
                break;
            }
        }

        sum += calibration;
    }

    println!("{sum}");
}

fn get_reversed_numbers(numbers: &[(&str, i32)]) -> Vec<(String, i32)> {
    let mut reversed = Vec::with_capacity(numbers.len());

    for (name, value) in numbers {
        reversed.push((name.chars().rev().collect::<String>(), *value));
    }

    reversed
}

fn build_trie<T: AsRef<str>>(entries: &[(T, i32)]) -> Trie {
    let mut trie = Trie::new();

    for (key, value) in entries {
        trie.insert(key.as_ref(), *value);
    }

    trie
}

#[cfg(test)]
mod tests {
    use crate::day_1::part_two::part_two;

    #[test]
    fn test_part_two() {
        part_two();
    }
}
