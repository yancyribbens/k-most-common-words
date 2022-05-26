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

    let k_arg = &args[1];
    let file_arg = &args[2];
    let k = k_arg.parse::<usize>().unwrap();

    let contents = fs::read_to_string(file_arg)
        .expect("Something went wrong reading the file");

    //let mut corpus = Corpus::new(contents);

    //let mut most_common_words: Vec<String> = Vec::new();
    //corpus.find_most_common_words(k, &mut most_common_words);

    //println!("{:?}", most_common_words);
}
