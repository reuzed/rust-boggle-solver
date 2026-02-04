use std::io::{BufRead, stdin};

use rust_boggle_solver::words::{is_prefix, is_word};

fn main() {
    let stdin = stdin();

    for line in stdin.lock().lines() {
        let word = line.unwrap();

        println!("{} {} {}", word, is_word(&word), is_prefix(&word));
    }
}