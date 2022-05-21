// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

use std::collections::HashMap;

use core::cmp::Ordering;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq, Ord)]
struct WordMeta {
    word: String,
    count: u8,
}

struct Words {
    words: Vec<WordMeta>
}

impl Words {
    fn new(word_table: HashMap<String, u8>) -> Self {
        let mut words: Vec<WordMeta> = Vec::new();

        let mut words_vec = word_table
            .iter()
            .collect::<Vec<(&String, &u8)>>();

        for w in words_vec {
          let word_meta = WordMeta {
            word: w.0.to_string(),
            count: *w.1
          };

          words.push(word_meta);
        }

        Self { words }
    }

    fn sort(&mut self) {
        self.words
            .sort_unstable_by_key(
                |item| (Reverse(item.count), item.word.clone())
            )
    }

    fn find_most_common_words(
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

impl PartialOrd for WordMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.count == other.count {
            return Some(self.word.cmp(&other.word));
        }
        Some(self.count.cmp(&other.count))
    }
}

fn build_word_table(words: String, mut word_table: HashMap<String, u8>) -> 
std::collections::HashMap<String, u8> {
    let w = words.split_whitespace();

    for word in w {
        let current_word: String = word
            .replace(|c: char| !c.is_ascii_alphabetic(), "")
            .to_lowercase();

        if current_word != "" {
            *word_table.entry(current_word).or_insert(0) += 1;
        }
    }

    word_table
}

fn main() {
    // find_most_common_words
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ops::Range;

    fn quote() -> String {
        String::from(r#"I'm Nobody! Who are you?
            Are you – Nobody – too?
            Then there's a pair of us!
            Don't tell! they'd advertise – you know!

            How dreary – to be – Somebody!
            How public – like a Frog –
            To tell one's name – the livelong June –
            To an admiring Bog!"#
        )
    }

    fn word_count_three() -> Vec<&'static str> {
        vec![
            "to",
            "you"
        ]
    }

    fn word_count_two() -> Vec<&'static str> {
        vec![
            "a",
            "are",
            "how",
            "nobody",
            "tell"
        ]
    }

    fn word_count_one() -> Vec<&'static str> {
        vec![
            "admiring",
            "advertise",
            "an",
            "be",
            "bog",
            "dont",
            "dreary",
            "frog",
            "im",
            "june",
            "know",
            "like",
            "livelong",
            "name",
            "of",
            "ones",
            "pair",
            "public",
            "somebody",
            "the",
            "then",
            "theres",
            "theyd",
            "too",
            "us",
            "who"
        ]
    }

    #[test]
    fn build_word_table_test() {
        let word_table = HashMap::new();
        let word_table = build_word_table(quote(), word_table);

        for w in word_count_three() {
            let count = word_table.get(w).unwrap();
            assert_eq!(*count, 3 as u8);
        }

        for w in word_count_two() {
            let count = word_table.get(w).unwrap();
            assert_eq!(*count, 2 as u8);
        }

        for w in word_count_one() {
            let count = word_table.get(w).unwrap();
            assert_eq!(*count, 1 as u8);
        }

        let count = word_table.get("");
        assert_eq!(None, count);
    }

    #[test]
    fn sort_word_list_test() {
        let word_table = HashMap::new();
        let word_table = build_word_table(quote(), word_table);
        let mut words = Words::new(word_table);
        words.sort();
        
        let mut expected = Vec::new();

        for w in word_count_three() {
            let word_meta = WordMeta { word: w.to_string(), count: 3 };
            expected.push(word_meta);
        }

        for w in word_count_two() {
            let word_meta = WordMeta { word: w.to_string(), count: 2 };
            expected.push(word_meta);
        }

        for w in word_count_one() {
            let word_meta = WordMeta { word: w.to_string(), count: 1 };
            expected.push(word_meta);
        }

        assert_eq!(expected, words.words);
    }

    #[test]
    fn find_most_common_words_test() {
        let word_table = HashMap::new();

        let words: Vec<WordMeta> = Vec::new();
        let mut words = Words { words };

        let word_table = build_word_table(quote(), word_table);
        let mut words = Words::new(word_table);

        let mut most_common_words: Vec<String> = Vec::new();
        words.find_most_common_words(1, &mut most_common_words);
        assert_eq!(vec!["to"], most_common_words);

        let mut most_common_words: Vec<String> = Vec::new();
        words.find_most_common_words(2, &mut most_common_words);
        assert_eq!(vec!["to", "you"], most_common_words);

        let mut most_common_words: Vec<String> = Vec::new();
        words.find_most_common_words(3, &mut most_common_words);
        assert_eq!(vec!["to","you","a"], most_common_words);
    }
}
