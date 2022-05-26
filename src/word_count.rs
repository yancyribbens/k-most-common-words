use core::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord)]
pub struct WordCount {
    pub word: String,
    pub count: u32,
}

impl WordCount {
    pub fn increment_counter(&mut self) {
        self.count += 1;
    }
}

impl PartialOrd for WordCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.count == other.count {
            return Some(self.word.cmp(&other.word));
        }
        Some(self.count.cmp(&other.count))
    }
}
