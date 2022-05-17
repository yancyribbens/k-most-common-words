// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

use std::collections::HashMap;

fn find_most_common_words(
    k: u8,
    word_table: std::collections::HashMap<String, u8>,
    mut results: Vec<String>) -> Vec<String> {

        let mut counts: Vec<(&String, &u8)> = word_table.iter().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1));

        for k in 0..k {
            results.push(counts[0].0.to_string());
        }

        results
}

fn build_word_table(words: String, mut word_table: std::collections::HashMap<String, u8>) -> 
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

        let count = word_table.get("");
        assert_eq!(None, count);
    }

    #[test]
    fn find_most_common_word() {
        let mut word_table = HashMap::new();
        let mut results = Vec::new();

        let word_table = build_word_table(quote(), word_table);
        let word = find_most_common_words(1, word_table, results);

        assert_eq!(word.len(), 1);
        //assert_eq!("to", word[0]);
    }
}
