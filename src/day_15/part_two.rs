use std::fs;

struct FocalLength {
    value: i32,
}

struct Box {
    lense_slots: Vec<FocalLength>,
}

struct Operation {
    operation: String,
    focal_length: i32,
}

impl Operation {
    fn new() -> Self {
        Operation {
            operation: String::new(),
            focal_length: 0,
        }
    }

    fn from_char(operation: char) -> Self {
        Operation {
            operation: String::from(operation),
            focal_length: 0,
        }
    }
}

#[allow(dead_code)]
fn part_two() {
    let input = fs::read_to_string("src/day_15/input.txt").unwrap();
    let initialization_sequence = input.as_str();
    let mut boxes = Vec::<Box>::with_capacity(256);

    for (label, operation) in initialization_sequence
        .split(",")
        .map(|step| get_operation(step))
    {}

    todo!()
}

fn get_operation(step: &str) -> (String, Operation) {
    let mut label = String::new();
    let mut operation = Operation::new();
    for char in step.chars() {
        if char.eq(&'-') {
            return (label, Operation::from_char('-'));
        }

        // if char.eq(&'=') {
        //         operation = Operation {
        //             operation: String::from('-'),
        //             focal_length:
        //         },
        //     );
        // }

        label.push(char);
    }

    todo!()
}
