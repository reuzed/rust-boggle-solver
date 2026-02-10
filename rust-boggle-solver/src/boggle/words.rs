use std::{
    collections::HashSet,
    fs::{self},
    sync::LazyLock,
};

pub fn load_words() -> HashSet<Box<str>> {
    let start = std::time::Instant::now();
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");
    let words: HashSet<Box<str>> = contents
        .lines()
        .filter(|w| w.len() <= 16)
        .map(Box::from) // &str -> Box<str> directly, no intermediate String
        .collect();
    println!("Built words hashmap in {:?}", start.elapsed());
    words
}

static WORDS: LazyLock<HashSet<Box<str>>> = LazyLock::new(load_words);

static WORD_PREFIXES: LazyLock<HashSet<Box<str>>> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let word_prefixes: HashSet<Box<str>> = WORDS
        .iter()
        .flat_map(|w| prefixes(w))
        .map(Box::from) // &str -> Box<str> directly
        .collect(); // collect straight into the HashSet, no intermediate Vec
    println!("Built word prefixes hashmap in {:?}", start.elapsed());
    word_prefixes
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
