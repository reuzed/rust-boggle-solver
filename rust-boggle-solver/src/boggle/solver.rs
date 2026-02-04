// To solve boggle, take a board as input
// Use simple DFS to find all paths in the boggle board, culling these when the prefixes do not occur 
// For this we need a word set - words.txt - https://github.com/dwyl/english-words/blob/master/words_alpha.txt
// Could try out https://github.com/wordset/wordset-dictionary

use super::board::{Board, Coord, coords_vec};

pub struct WordRecord {
    path: Vec<Coord>,
    word: String,
}

pub fn solve(board: Board) -> Vec<WordRecord> {
    let start_coords = coords_vec();

    panic!()
}
