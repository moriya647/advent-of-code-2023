use std::fs;

#[allow(dead_code)]
fn part_one() {
    let input = fs::read_to_string("src/day_3/full_engine_schematic.txt").unwrap();
    let mut sum = 0;
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (i, line) in schematic.iter().enumerate() {
        let mut part = 0;
        let mut is_part = false;
        for (j, value) in line.iter().enumerate() {
            if value.is_ascii_digit() {
                part = part * 10 + value.to_digit(10).unwrap();

                if check_is_part(&schematic, i, j) {
                    is_part = true;
                }
            } else if is_part {
                sum += part;
                part = 0;
                is_part = false;
            }

            if j == line.len() - 1 && is_part {
                sum += part;
            } else if !value.is_ascii_digit() {
                part = 0;
                is_part = false;
            }
        }
    }

    println!("{sum}");
}

fn check_is_part(schematic: &Vec<Vec<char>>, line: usize, index: usize) -> bool {
    let mut is_part = false;
    let left = schematic[line][index.saturating_sub(1)];
    if is_symbol(left) {
        is_part = true;
    };
    if let Some(right) = schematic[line].get(index.saturating_add(1)) {
        if is_symbol(*right) {
            is_part = true;
        }
    }

    if line == 0 && schematic.len() > 1 {
        if check_left_right_center(schematic, line.saturating_add(1), index) {
            is_part = true;
        };
    } else if line == schematic.len() - 1 {
        if check_left_right_center(schematic, line.saturating_sub(1), index) {
            is_part = true;
        };
    } else {
        if check_left_right_center(schematic, line.saturating_sub(1), index) {
            is_part = true
        };
        if check_left_right_center(schematic, line.saturating_add(1), index) {
            is_part = true;
        };
    }

    is_part
}

fn check_left_right_center(schematic: &[Vec<char>], line: usize, center_index: usize) -> bool {
    let mut is_part = false;

    if let Some(left) = schematic[line].get(center_index.saturating_sub(1)) {
        if is_symbol(*left) {
            is_part = true;
        };
    }

    if let Some(center) = schematic[line].get(center_index) {
        if is_symbol(*center) {
            is_part = true;
        }
    }

    if let Some(right) = schematic[line].get(center_index.saturating_add(1)) {
        if is_symbol(*right) {
            is_part = true;
        }
    }

    is_part
}

fn is_symbol(value: char) -> bool {
    !value.is_ascii_digit() && value != '.' && value != '\n'
}

#[cfg(test)]
mod tests {
    use crate::day_3::part_one::part_one;

    #[test]
    fn test_part_one() {
        part_one();
    }
}
