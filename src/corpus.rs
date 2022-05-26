
use crate::word_count::WordCount;
use std::collections::HashMap;

use std::cmp::Reverse;

#[derive(Debug)]
pub struct Corpus {
    words: Vec<WordCount>
}

impl Corpus {
    fn build_table(table: &mut HashMap<String, WordCount>, words: &str) {
        for w in words.split_whitespace() {
            let current: String = w
                .replace(|c: char| !c.is_ascii_alphabetic(), "")
                .to_lowercase();

            let word_count = WordCount::new(&current, 0);

            let entry = table.entry(current).or_insert(word_count);
            entry.increment_counter();
        }

        table.retain(|k, _| k != "");
    }

    pub fn get_word_count(&self, w: &WordCount) -> usize {
        w.get_count()
    }

    pub fn get_name(&self, w: &WordCount) -> String {
        w.get_name()
    }

    pub fn new(words: &str) -> Self {
        let mut table = HashMap::new();
        Self::build_table(&mut table, words);
        let words: Vec<WordCount> = table.into_values().collect();
        Self { words }
    }

    fn sort(&mut self) {
        self.words
            .sort_unstable_by_key(
                |item| (Reverse(item.get_count()), item.get_name().clone())
            )
    }

    pub fn find_most_common_words(
        &mut self,
        k: usize,
        most_common_words: &mut Vec<String>) {
        self.sort();

        for i in 0..k {
            let w = self.words[i].get_name().clone();
            most_common_words.push(w);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn corpus_with_two_words_test() {
        let mut corpus = Corpus::new("lorem ipsum");
        assert_eq!(2, corpus.words.len());
        corpus.sort();

        let first_word = &corpus.words[0];
        let second_word = &corpus.words[1];

        assert_eq!(1, first_word.get_count());
        assert_eq!("ipsum".to_string(), first_word.get_name());

        assert_eq!(1, second_word.get_count());
        assert_eq!("lorem".to_string(), second_word.get_name());
    }

    #[test]
    fn corpus_with_capitol_words_test() {
        let mut corpus = Corpus::new("Lorem IpSum");
        assert_eq!(2, corpus.words.len());
        corpus.sort();

        let first_word = &corpus.words[0];
        let second_word = &corpus.words[1];

        assert_eq!(1, first_word.get_count());
        assert_eq!("ipsum".to_string(), first_word.get_name());

        assert_eq!(1, second_word.get_count());
        assert_eq!("lorem".to_string(), second_word.get_name());
    }

    #[test]
    fn corpus_with_punctuation_test() {
        let mut corpus = Corpus::new("lorem, ipsum");
        assert_eq!(2, corpus.words.len());
        corpus.sort();

        let first_word = &corpus.words[0];
        let second_word = &corpus.words[1];

        assert_eq!(1, first_word.get_count());
        assert_eq!("ipsum".to_string(), first_word.get_name());

        assert_eq!(1, second_word.get_count());
        assert_eq!("lorem".to_string(), second_word.get_name());
    }

    #[test]
    fn corpus_with_punctuation_surrounded_by_spaces_test() {
        let mut corpus = Corpus::new("lorem -- ipsum");
        assert_eq!(2, corpus.words.len());
        corpus.sort();

        let first_word = &corpus.words[0];
        let second_word = &corpus.words[1];

        assert_eq!(1, first_word.get_count());
        assert_eq!("ipsum".to_string(), first_word.get_name());

        assert_eq!(1, second_word.get_count());
        assert_eq!("lorem".to_string(), second_word.get_name());
    }

    //#[test]
    //fn sort_word_list_test() {
        //let mut corpus = Corpus::new(quote());
        //corpus.sort();

        //let mut expected = Vec::new();
        //for word in word_count_three() {
            //let word_count = WordCount { word: word.to_string(), count: 3 };
            //expected.push(word_count);
        //}

        //for word in word_count_two() {
            //let word_count = WordCount { word: word.to_string(), count: 2 };
            //expected.push(word_count);
        //}

        //for word in word_count_one() {
            //let word_count = WordCount { word: word.to_string(), count: 1 };
            //expected.push(word_count);
        //}

        //assert_eq!(expected, corpus.words);
    //}

    //#[test]
    //fn find_most_common_words_test() {
        //let mut corpus = Corpus::new(quote());

        //let mut most_common_words: Vec<String> = Vec::new();
        //corpus.find_most_common_words(1, &mut most_common_words);
        //assert_eq!(vec!["to"], most_common_words);

        //let mut most_common_words: Vec<String> = Vec::new();
        //corpus.find_most_common_words(2, &mut most_common_words);
        //assert_eq!(vec!["to", "you"], most_common_words);

        //let mut most_common_words: Vec<String> = Vec::new();
        //corpus.find_most_common_words(3, &mut most_common_words);
        //assert_eq!(vec!["to","you","a"], most_common_words);
    //}
}
