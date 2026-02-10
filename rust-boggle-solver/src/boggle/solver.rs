// To solve boggle, take a board as input
// Use simple DFS to find all paths in the boggle board, culling these when the prefixes do not occur
// For this we need a word set - words.txt - https://github.com/dwyl/english-words/blob/master/words_alpha.txt
// Could try out https://github.com/wordset/wordset-dictionary

use serde::Serialize;

use super::{
    board::{coords_vec, neighbouring_coords, Board, Coord},
    words::{is_prefix, is_word},
};

#[derive(Debug, Serialize)]
pub struct WordRecord {
    pub path: Vec<Coord>,
    pub word: String,
}

pub fn solve(board: &Board) -> Vec<WordRecord> {
    let start_coords = coords_vec();

    // From each start position on the boggle board, follow all adjancencies
    // At each step, check if the current path's word is a prefix of some word
    // If not a prefix we can return early, if path is a word, add to output

    let mut word_records: Vec<WordRecord> = Vec::new();
    let mut paths: Vec<Vec<Coord>> = Vec::new();
    for start in start_coords.into_iter() {
        paths.push(vec![start]);
    }
    for _ in 0..16 {
        paths = paths.iter().flat_map(|p| extend_paths(&board, p)).collect();
        for path in &paths {
            let word = board.path_word(path);
            if is_word(&word) {
                word_records.push(WordRecord {
                    path: path.to_vec(),
                    word,
                });
            }
        }
    }
    word_records
}

fn extend_paths(board: &Board, path: &Vec<Coord>) -> Vec<Vec<Coord>> {
    let path_end = path[path.len() - 1];

    let mut extended_paths: Vec<Vec<Coord>> = Vec::new();
    for path_next in neighbouring_coords(path_end)
        .into_iter()
        .filter(|c| !path.contains(c))
    {
        let new_path = {
            let mut p = path.clone();
            p.push(path_next);
            p
        };
        if is_prefix(&board.path_word(&new_path)) {
            extended_paths.push(new_path);
        }
    }

    return extended_paths;
}
