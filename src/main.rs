// https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf

// Given a text file and an integer k, print the k most
// common words in the file (and the number of
// their occurrences) in decreasing frequency.

fn find_most_common_words(k: u8, words: String) -> String {
    String::from("to")
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
    fn find_most_common_word() {
        let most_common_word = find_most_common_words(1, quote());

        assert_eq!(most_common_word, "to".to_string());
    }
}
