use std::{collections::{HashSet}, fs, sync::LazyLock};
use rustc_hash::FxHashSet;

static WORDS: LazyLock<FxHashSet<String>> = LazyLock::new(||{
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");
    let start = std::time::Instant::now();

    let a = contents.lines().filter(|w| w.len() <= 16).map(|s| s.to_string()).collect();
    println!("Built words hashmap in {:?}", start.elapsed());
    a

});

static WORD_PREFIXES: LazyLock<FxHashSet<Box<str>>> = LazyLock::new(||{
    let start = std::time::Instant::now();
    let a = WORDS.iter()
        .flat_map(|w| prefixes(w))
        .map(|s| s.to_string().into_boxed_str())
        .collect();
    println!("Built word prefixes hashmap in {:?}", start.elapsed());
    a
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