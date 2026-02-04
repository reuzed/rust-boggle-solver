use std::{collections::{HashMap, HashSet}, fs};

fn load_words() -> HashSet<String> {
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");

    contents.trim().split("\n").map(|s| s.to_string()).collect()
}

pub fn is_word(word: &str) -> bool {
    let words = load_words();

    words.contains(word)
}