use std::{collections::{HashSet}, fs, sync::LazyLock};

static WORDS: LazyLock<HashSet<String>> = LazyLock::new(||{
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");

    contents.lines().filter(|w| w.len() <= 16).map(|s| s.to_string()).collect()
});

static WORD_PREFIXES: LazyLock<HashSet<String>> = LazyLock::new(||{
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");

    contents.lines().filter(|w| w.len() <= 16).flat_map(|w| prefixes(w)).map(|s| s.to_string()).collect()
});

fn prefixes(word: &str) -> Vec<String> {
    // For example: dog -> {d, do, dog}
    (1..word.len()).map(|i| word[..i].to_string()).collect()
}

pub fn is_word(word: &str) -> bool {
    WORDS.contains(word)
}

pub fn is_prefix(word: &str) -> bool {
    WORD_PREFIXES.contains(word)
}