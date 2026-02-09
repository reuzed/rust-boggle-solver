use std::{collections::HashSet, fs::{self, File}, sync::LazyLock};

static WORDS: LazyLock<HashSet<String>> = LazyLock::new(||{
 
    let mut words: HashSet<String> = HashSet::with_capacity(364247);

    let start = std::time::Instant::now();
    let contents = fs::read_to_string("words.txt").expect("words.txt exists");
    words.extend(contents.lines().filter(|w| w.len() <= 16).map(|s| s.to_string()));
    println!("Built words hashmap in {:?}", start.elapsed());
    words

});

static WORD_PREFIXES: LazyLock<HashSet<Box<str>>> = LazyLock::new(||{
    let start = std::time::Instant::now();
    let mut word_prefixes: HashSet<Box<str>> = HashSet::with_capacity(5 * 364247);
    word_prefixes.extend(WORDS.iter()
        .flat_map(|w| prefixes(w))
        .map(|s| s.to_string().into_boxed_str())
        .collect::<Vec<Box<str>>>());
    // let a = WORDS.iter()
    //     .flat_map(|w| prefixes(w))
    //     .map(|s| s.to_string().into_boxed_str())
    //     .collect();
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