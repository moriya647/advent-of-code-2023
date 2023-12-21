use std::fs;

use crate::day_2::game::Game;

#[allow(dead_code)]
fn part_two() {
    let input = fs::read_to_string("src/day_2/full_input.txt").unwrap();
    let mut power = 0;

    for game in input.lines().map(to_game()) {
        let max_set = game.parse_max_set();
        power += max_set.green * max_set.red * max_set.blue;
    }

    println!("{power}");
}

fn to_game() -> fn(&str) -> Game {
    |line| Game { record: line }
}

#[cfg(test)]
mod tests {
    use crate::day_2::part_two::part_two;

    #[test]
    fn test_part_two() {
        part_two();
    }
}
