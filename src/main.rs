// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

pub mod corpus;
pub mod word_count;

use crate::corpus::Corpus;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_arg = &args[1];
    let k_arg = &args[2];
    let k = k_arg.parse::<usize>().unwrap();

    let contents = fs::read_to_string(file_arg)
        .expect("Something went wrong reading the file");

    let mut corpus = Corpus::new(contents);

    let mut most_common_words: Vec<String> = Vec::new();
    corpus.find_most_common_words(k, &mut most_common_words);

    println!("{:?}", most_common_words);
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
        let mut table = HashMap::new();

        Corpus::build_table(&mut table, quote());
        for word in word_count_three() {
            let count = table.get(word).unwrap();
            let expected_word_count = WordCount {
                word: String::from(word), count: 3 as u8 
            };
            assert_eq!(*count, expected_word_count);
        }

        for word in word_count_two() {
            let count = table.get(word).unwrap();
            let expected_word_count = WordCount {
                word: String::from(word), count: 2 as u8
            };
            assert_eq!(*count, expected_word_count);
        }

        for word in word_count_one() {
            let count = table.get(word).unwrap();
            let expected_word_count = WordCount {
                word: String::from(word), count: 1 as u8 };
            assert_eq!(*count, expected_word_count);
        }

        let count = table.get("");
        assert_eq!(None, count);
    }

    #[test]
    fn sort_word_list_test() {
        let mut corpus = Corpus::new(quote());
        corpus.sort();

        let mut expected = Vec::new();
        for word in word_count_three() {
            let word_count = WordCount { word: word.to_string(), count: 3 };
            expected.push(word_count);
        }

        for word in word_count_two() {
            let word_count = WordCount { word: word.to_string(), count: 2 };
            expected.push(word_count);
        }

        for word in word_count_one() {
            let word_count = WordCount { word: word.to_string(), count: 1 };
            expected.push(word_count);
        }

        assert_eq!(expected, corpus.words);
    }

    #[test]
    fn find_most_common_words_test() {
        let mut corpus = Corpus::new(quote());

        let mut most_common_words: Vec<String> = Vec::new();
        corpus.find_most_common_words(1, &mut most_common_words);
        assert_eq!(vec!["to"], most_common_words);

        let mut most_common_words: Vec<String> = Vec::new();
        corpus.find_most_common_words(2, &mut most_common_words);
        assert_eq!(vec!["to", "you"], most_common_words);

        let mut most_common_words: Vec<String> = Vec::new();
        corpus.find_most_common_words(3, &mut most_common_words);
        assert_eq!(vec!["to","you","a"], most_common_words);
    }
}
