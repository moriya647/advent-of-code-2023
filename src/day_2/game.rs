use crate::day_2::cube_set::CubeSet;

pub(crate) struct Game<'a> {
    pub(crate) record: &'a str,
}

impl<'a> Game<'a> {
    pub(crate) fn parse_max_set(&self) -> CubeSet {
        let mut current_number = 0;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for c in self.record.chars() {
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
}
