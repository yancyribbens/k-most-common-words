// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

use std::collections::HashMap;

fn build_sorted_vec(word_table: HashMap<String, u8>, word_vec: &mut Vec<(String, u8)>) {
    let mut sort_word_vec = word_table
        .iter()
        .collect::<Vec<(&String, &u8)>>();

    sort_word_vec.sort_by(|a, b| b.1.cmp(a.1));

    for word in sort_word_vec {
        word_vec.push((word.0.to_string(), *word.1));
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

    fn assert_word_sort(j: Range<usize>, expected_occurence: Vec<(&str, u8)>, sorted_vec: &Vec<(String, u8)>) {
        let mut occurence = Vec::new();
        for i in j { 
          occurence.push(&sorted_vec[i]); 
        }
        occurence.sort();

        for i in 0..expected_occurence.len() {
          assert_eq!(expected_occurence[i].0, occurence[i].0);
          assert_eq!(expected_occurence[i].1, occurence[i].1);
        }
    }

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
        let expected_occurence_three = vec![("to", 3), ("you", 3)];
        let expected_occurence_two = vec![("a", 2), ("are", 2), ("how", 2), ("nobody", 2), ("tell", 2)];
        let expected_occurence_one = vec![
          ("admiring", 1),
          ("advertise", 1),
          ("an", 1),
          ("be", 1),
          ("bog", 1),
          ("dont", 1),
          ("dreary", 1),
          ("frog", 1),
          ("im", 1),
          ("june", 1),
          ("know", 1),
          ("like", 1),
          ("livelong", 1),
          ("name", 1),
          ("of", 1),
          ("ones", 1),
          ("pair", 1),
          ("public", 1),
          ("somebody", 1),
          ("the", 1),
          ("then", 1),
          ("theres", 1),
          ("theyd", 1),
          ("too", 1),
          ("us", 1),
          ("who", 1),
        ];

        let word_table = HashMap::new();
        let mut sorted_vec = Vec::new();
        let word_table = build_word_table(quote(), word_table);
        build_sorted_vec(word_table, &mut sorted_vec);

        assert_word_sort(0..2, expected_occurence_three, &sorted_vec);
        assert_word_sort(2..7, expected_occurence_two, &sorted_vec);
        assert_word_sort(7..33, expected_occurence_one, &sorted_vec);
    }
}
