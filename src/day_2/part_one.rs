use std::fs;

use crate::day_2::cube_set::CubeSet;
use crate::day_2::game::Game;

#[allow(dead_code)]
fn part_one() {
    let input = fs::read_to_string("src/day_2/full_input.txt").unwrap();
    let max_cubes = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut sum = 0;

    for (i, game) in input.lines().map(|line| Game { record: line }).enumerate() {
        if game.parse_max_set() <= max_cubes {
            sum += i + 1;
        }
    }

    println!("{sum}")
}

#[cfg(test)]
mod tests {
    use crate::day_2::part_one::part_one;

    #[test]
    fn test_part_one() {
        part_one()
    }
}
