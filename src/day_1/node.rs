use std::collections::HashMap;

pub(crate) struct Node {
    pub(crate) children: HashMap<char, Node>,
    pub(crate) last: bool,
    pub(crate) value: Option<i32>,
}

impl Node {
    pub(crate) fn new() -> Self {
        Node {
            children: HashMap::new(),
            last: false,
            value: None,
        }
    }
}
