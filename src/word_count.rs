use core::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord)]
pub struct WordCount {
    name: String,
    count: usize,
}

impl WordCount {
    pub fn new(word: &str, count: usize) -> Self {
        Self { name: word.to_string(), count }
    }

    pub fn increment_counter(&mut self) {
        self.count += 1;
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl PartialOrd for WordCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.count == other.count {
            return Some(self.name.cmp(&other.name));
        }
        Some(self.count.cmp(&other.count))
    }
}
