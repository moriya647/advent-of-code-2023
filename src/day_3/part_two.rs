use std::collections::VecDeque;
use std::fs;

#[allow(dead_code)]
fn part_two() {
    let input = fs::read_to_string("src/day_3/full_engine_schematic.txt").unwrap();
    let mut lines = input.lines();
    let mut lines_window = VecDeque::<Vec<char>>::with_capacity(3);

    for _ in 0..3 {
        if let Some(line) = lines.next() {
            lines_window.push_back(line.chars().collect::<Vec<char>>());
        }
    }

    let mut ratio_sum = 0;
    let mut first_line = true;
    let empty = vec![];

    while let Some(current_line) = lines_window.pop_front() {
        let first = &current_line;
        let second = lines_window.front().unwrap_or(&empty);
        let third = lines_window.get(1).unwrap_or(&empty);

        if first_line {
            if let Some(ratio) = get_line_ratio(&empty, first, second) {
                ratio_sum += ratio;
            }

            first_line = false;
        }

        if !first.is_empty() && !second.is_empty() && !third.is_empty() {
            if let Some(ratio) = get_line_ratio(first, second, third) {
                ratio_sum += ratio;
            }
        }

        if !first.is_empty() && !second.is_empty() && third.is_empty() {
            if let Some(ratio) = get_line_ratio(first, second, &empty) {
                ratio_sum += ratio;
            }
        }

        if let Some(next_line) = lines.next() {
            lines_window.push_back(next_line.chars().collect::<Vec<char>>());
        }
    }

    println!("{ratio_sum}");
}

fn get_line_ratio(previous: &Vec<char>, line: &Vec<char>, next: &Vec<char>) -> Option<u32> {
    let mut ratio = 0;

    for (i, &value) in line.iter().enumerate() {
        if value == '*' {
            let parts = get_surrounding_parts(previous, line, next, i);

            if parts.len() == 2 {
                ratio += parts[0] * parts[1];
            }
        }
    }

    if ratio == 0 {
        return None;
    }

    Some(ratio)
}

fn get_surrounding_parts(
    previous: &Vec<char>,
    line: &Vec<char>,
    next: &Vec<char>,
    position: usize,
) -> Vec<u32> {
    let mut parts = vec![];

    if line.is_empty() {
        return parts;
    }

    parts.append(&mut get_adjacent_parts(line, position));

    if !previous.is_empty() {
        parts.append(&mut get_adjacent_parts(previous, position));
    }

    if !next.is_empty() {
        parts.append(&mut get_adjacent_parts(next, position));
    }

    parts
}

fn get_adjacent_parts(line: &[char], position: usize) -> Vec<u32> {
    let mut parts = vec![];
    let mut left = None;
    let mut mid = None;
    let mut right = None;

    if let Some(chars) = line.get(..=position.saturating_sub(1)) {
        let mut digits = chars
            .iter()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        digits.reverse();

        if !digits.is_empty() {
            left = Some(digits);
        }
    }

    if let Some(value) = line.get(position) {
        if value.is_ascii_digit() {
            mid = Some(value.to_digit(10).unwrap());
        }
    }

    if let Some(chars) = line.get(position.saturating_add(1)..) {
        let digits = chars
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        if !digits.is_empty() {
            right = Some(digits);
        }
    }

    if let Some(mid_digit) = mid {
        if let Some(left_digits) = left {
            let left_mid_digits = [left_digits, vec![mid_digit]].concat();
            if let Some(right_digits) = right {
                parts.push(join(&[left_mid_digits, right_digits].concat()));
            } else {
                parts.push(join(&left_mid_digits));
            }
        } else if let Some(right_digits) = right {
            parts.push(join(&[vec![mid_digit], right_digits].concat()));
        } else {
            parts.push(mid_digit);
        }
    } else {
        if let Some(left_digits) = left {
            parts.push(join(&left_digits));
        }
        if let Some(right_digits) = right {
            parts.push(join(&right_digits));
        }
    }

    parts
}

fn join<'a, T: IntoIterator<Item = &'a u32>>(numbers: T) -> u32 {
    let mut value = 0;

    for &number in numbers {
        let digits = if number == 0 {
            1
        } else {
            (number as f64).log10().trunc() as u32 + 1
        };

        value = value * 10u32.pow(digits) + number;
    }

    value
}

#[cfg(test)]
mod tests {
    use crate::day_3::part_two::part_two;

    #[test]
    fn test_part_two() {
        part_two();
    }
}
