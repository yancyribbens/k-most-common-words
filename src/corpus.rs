
use crate::word_count::WordCount;
use std::collections::HashMap;

use std::cmp::Reverse;

pub struct Corpus {
    words: Vec<WordCount>
}

impl Corpus {
    fn build_table(table: &mut HashMap<String, WordCount>, words: String) {
        for w in words.split_whitespace() {
            let current: String = w
                .replace(|c: char| !c.is_ascii_alphabetic(), "")
                .to_lowercase();

            let word_meta = WordCount {
                word: current.clone(),
                count: 0
            };

            let entry = table.entry(current).or_insert(word_meta);
            entry.increment_counter();
        }

        table.retain(|k, _| k != "");
    }

    pub fn new(words: String) -> Self {
        let mut table = HashMap::new();
        Self::build_table(&mut table, words);
        let words: Vec<WordCount> = table.into_values().collect();
        Self { words }
    }

    fn sort(&mut self) {
        self.words
            .sort_unstable_by_key(
                |item| (Reverse(item.count), item.word.clone())
            )
    }

    pub fn find_most_common_words(
        &mut self,
        k: usize,
        most_common_words: &mut Vec<String>) {
        self.sort();

        for i in 0..k {
            let w = self.words[i].word.clone();
            most_common_words.push(w);
        }
    }
}
