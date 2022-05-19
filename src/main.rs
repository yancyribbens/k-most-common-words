// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

use std::collections::HashMap;

use core::cmp::Ordering;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq, Ord)]
struct WordCount {
    word: String,
    count: u8,
}

impl PartialOrd for WordCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.count == other.count {
            return Some(self.word.cmp(&other.word));
        }
        Some(self.count.cmp(&other.count))
    }
}

fn build_sorted_vec(word_table: HashMap<String, u8>, word_vec: &mut Vec<WordCount>) {
    let mut sort_word_vec = word_table
        .iter()
        .collect::<Vec<(&String, &u8)>>();

    for w in sort_word_vec {
      let word_count = WordCount {
        word: w.0.to_string(),
        count: *w.1
      };
      word_vec.push(word_count);
    }

    word_vec.sort_unstable_by_key(|item| (Reverse(item.count), item.word.clone()))
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

    #[test]
    fn build_word_table_test() {
        let word_table = HashMap::new();
        let word_table = build_word_table(quote(), word_table);

        let count = word_table.get("im").unwrap();
        assert_eq!(*count, 1 as u8);

        let count = word_table.get("I'm");
        assert_eq!(None, count);

        let count = word_table.get("goldenrod");
        assert_eq!(None, count);

        let count = word_table.get("nobody").unwrap();
        assert_eq!(*count, 2 as u8);

        let count = word_table.get("");
        assert_eq!(None, count);
    }

    #[test]
    fn build_sorted_vec_test() {
        let expected_occurence = vec![
            WordCount {
                word: "to".to_string(),
                count: 3
            },
            WordCount {
                word: "you".to_string(),
                count: 3,
            },
            WordCount {
                word: "a".to_string(),
                count: 2,
            },
            WordCount {
                word: "are".to_string(),
                count: 2,
            },
            WordCount {
                word: "how".to_string(),
                count: 2,
            },
            WordCount {
                word: "nobody".to_string(),
                count: 2,
            },
            WordCount {
                word: "tell".to_string(),
                count: 2,
            },
            WordCount {
                word: "admiring".to_string(),
                count: 1,
            },
            WordCount {
                word: "advertise".to_string(),
                count: 1,
            },
            WordCount {
                word: "an".to_string(),
                count: 1,
            },
            WordCount {
                word: "be".to_string(),
                count: 1,
            },
            WordCount {
                word: "bog".to_string(),
                count: 1,
            },
            WordCount {
                word: "dont".to_string(),
                count: 1,
            },
            WordCount {
                word: "dreary".to_string(),
                count: 1,
            },
            WordCount {
                word: "frog".to_string(),
                count: 1,
            },
            WordCount {
                word: "im".to_string(),
                count: 1,
            },
            WordCount {
                word: "june".to_string(),
                count: 1,
            },
            WordCount {
                word: "know".to_string(),
                count: 1,
            },
            WordCount {
                word: "like".to_string(),
                count: 1,
            },
            WordCount {
                word: "livelong".to_string(),
                count: 1,
            },
            WordCount {
                word: "name".to_string(),
                count: 1,
            },
            WordCount {
                word: "of".to_string(),
                count: 1,
            },
            WordCount {
                word: "ones".to_string(),
                count: 1,
            },
            WordCount {
                word: "pair".to_string(),
                count: 1,
            },
            WordCount {
                word: "public".to_string(),
                count: 1,
            },
            WordCount {
                word: "somebody".to_string(),
                count: 1,
            },
            WordCount {
                word: "the".to_string(),
                count: 1,
            },
            WordCount {
                word: "then".to_string(),
                count: 1,
            },
            WordCount {
                word: "theres".to_string(),
                count: 1,
            },
            WordCount {
                word: "theyd".to_string(),
                count: 1,
            },
            WordCount {
                word: "too".to_string(),
                count: 1,
            },
            WordCount {
                word: "us".to_string(),
                count: 1,
            },
            WordCount {
                word: "who".to_string(),
                count: 1,
            }
        ];

        let word_table = HashMap::new();
        let mut sorted_vec = Vec::new();
        let word_table = build_word_table(quote(), word_table);
        build_sorted_vec(word_table, &mut sorted_vec);

        for i in 0..expected_occurence.len() {
            assert_eq!(expected_occurence[i], sorted_vec[i]);
        }
    }
}
