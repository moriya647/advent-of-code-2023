use std::fs;

use crate::day_2::cube_set::CubeSet;

#[allow(dead_code)]
fn part_one() {
    let input = fs::read_to_string("src/day_2/full_input.txt").unwrap();
    let max_cubes = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut sum = 0;

    for (i, game) in input.lines().enumerate() {
        if parse_max_set(game) <= max_cubes {
            sum += i + 1;
        }
    }

    println!("{sum}")
}

fn parse_max_set(game: &str) -> CubeSet {
    let mut current_number = 0;
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for c in game.chars() {
        match c {
            ':' | ';' => current_number = 0,
            '0'..='9' => current_number = current_number * 10 + c.to_digit(10).unwrap(),
            'a'..='z' | 'A'..='Z' => match c {
                'r' => {
                    max_red = max_red.max(current_number);
                    current_number = 0;
                }
                'g' => {
                    max_green = max_green.max(current_number);
                    current_number = 0;
                }
                'b' => {
                    max_blue = max_blue.max(current_number);
                    current_number = 0;
                }
                _ => {}
            },
            _ => {}
        }
    }

    CubeSet {
        red: max_red as i32,
        green: max_green as i32,
        blue: max_blue as i32,
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::part_one::part_one;

    #[test]
    fn test_part_one() {
        part_one()
    }
}
