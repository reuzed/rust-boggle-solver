// To solve boggle, take a board as input
// Use simple DFS to find all paths in the boggle board, culling these when the prefixes do not occur 
// For this we need a word set - words.txt - https://github.com/dwyl/english-words/blob/master/words_alpha.txt
// Could try out https://github.com/wordset/wordset-dictionary

use super::{board::{Board, Coord, coords_vec, neighbouring_coords}, words::is_prefix};

pub struct WordRecord {
    path: Vec<Coord>,
    word: String,
}

pub fn solve(board: Board) -> Vec<WordRecord> {
    let start_coords = coords_vec();

    // From each start position on the boggle board, follow all adjancencies
    // At each step, check if the current path's word is a prefix of some word
    // If not a prefix we can return early, if path is a word, add to output
    for start in start_coords.into_iter() {

    }
    panic!()
}

fn partial_solve(board: Board, path: Vec<Coord>) -> Vec<Vec<Coord>> {
    let path_end = path[path.len()-1];

    let mut extended_paths: Vec<Vec<Coord>> = Vec::new();
    for path_next in neighbouring_coords(path_end).into_iter()
      .filter(
      |c| !path.contains(c)  
    ) {
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
