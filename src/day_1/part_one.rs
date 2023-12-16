use std::fs;

#[allow(dead_code)]
fn part_one() {
    let input = fs::read_to_string("day_1_input.txt");

    match input {
        Ok(value) => two_pointer_approach(&value),
        Err(e) => println!("Error: {e}"),
    }
}

fn two_pointer_approach(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        println!("{line}");
        let mut calibration = 0;

        for char in line.chars() {
            if char.is_numeric() {
                calibration += char.to_digit(10).unwrap() * 10;
                println!("left: {calibration}");
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                calibration += char.to_digit(10).unwrap();
                println!("right: {calibration}");
                break;
            }
        }

        sum += calibration;
    }

    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use crate::day_1::part_one::part_one;

    #[test]
    fn test_part_one() {
        part_one();
    }
}
