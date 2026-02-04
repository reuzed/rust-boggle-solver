use std::{collections::{HashSet}, fs, sync::LazyLock};

static WORDS: LazyLock<HashSet<String>> = LazyLock::new(||{
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");

    contents.lines().map(|s| s.to_string()).collect()
});

pub fn is_word(word: &str) -> bool {
    WORDS.contains(word)
}