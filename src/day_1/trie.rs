use crate::day_1::node::Node;

pub(crate) struct Trie {
    pub(crate) root: Node,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie { root: Node::new() }
    }
}

impl Trie {
    pub(crate) fn insert(&mut self, word: &str, value: i32) {
        let mut current = &mut self.root;

        for char in word.chars() {
            current = current.children.entry(char).or_insert(Node::new())
        }

        current.last = true;
        current.value = Some(value)
    }

    pub(crate) fn find_first_largest(&self, key: &str) -> Option<i32> {
        let mut hits = Vec::<&Node>::new();

        if find_first_largest(&self.root, key, 0, &mut hits) {
            return hits.last().unwrap().value;
        }

        None
    }
}

fn find_first_largest<'a>(
    root: &'a Node,
    key: &str,
    depth: usize,
    hits: &mut Vec<&'a Node>,
) -> bool {
    if depth >= key.len() {
        return false;
    }

    for (value, current) in root.children.iter() {
        if *value != key.chars().nth(depth).unwrap() {
            continue;
        }

        if current.last {
            println!("hit: {value}");
            hits.push(current)
        }

        if !find_first_largest(current, key, depth + 1, hits) {
            return !hits.is_empty();
        }
    }

    !hits.is_empty()
}
