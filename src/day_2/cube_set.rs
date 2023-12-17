use std::cmp::Ordering;

#[derive(PartialEq)]
pub(crate) struct CubeSet {
    pub(crate) red: i32,
    pub(crate) green: i32,
    pub(crate) blue: i32,
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(Ordering::Less)
        } else if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
