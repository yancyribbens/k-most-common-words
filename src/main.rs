// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

use std::collections::HashMap;

fn find_most_common_words(k: u8, words: String) -> String {
    String::from("to")
}

fn build_word_table(words: String, mut word_table: std::collections::HashMap<String, u8>) -> 
std::collections::HashMap<String, u8> {
    let w = words.split_whitespace();

    for word in w {
        let current_word: String = word
            .replace(|c: char| !c.is_ascii_alphabetic(), "")
            .to_lowercase();

        let count = word_table.get(&current_word);

        match count {
            Some(c) => {
                let new_count = c + 1;
                word_table.insert(current_word, new_count);
            },
            None => {
                word_table.insert(current_word, 1);
            }
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
        let mut word_table = HashMap::new();
        let word_table = build_word_table(quote(), word_table);

        let count = word_table.get("im").unwrap();
        assert_eq!(*count, 1 as u8);

        let count = word_table.get("I'm");
        assert_eq!(None, count);

        let count = word_table.get("goldenrod");
        assert_eq!(None, count);

        let count = word_table.get("nobody").unwrap();
        assert_eq!(*count, 2 as u8);
    }

    #[test]
    fn find_most_common_word() {
        let most_common_word = find_most_common_words(1, quote());
        assert_eq!(most_common_word, "to".to_string());
    }
}
