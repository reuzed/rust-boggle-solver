// Generate boards with long words present, through random selection and testing of the boards.

use crate::{board::random_board, solver::solve};

use super::{board::Board, solver::WordRecord};

pub type GenerateBoardResult = Vec<(Board, Vec<WordRecord>)>;

pub fn generate_boards(quantity: usize, max_attempts: u64) -> GenerateBoardResult{
    let mut best_boards: GenerateBoardResult = Vec::new();
    let mut max_length = 0;
    for _ in 0..max_attempts {
        let board = random_board();
        let solns = solve(&board);
        let longest = solns.iter().map(|wr| wr.word.len()).max();
        match longest {
            None => (),
            Some(l) => {
                if l >= max_length {
                    best_boards.push((board, solns));
                    max_length=l;
                }
            }
        }
    }
    best_boards.sort_by(
        |(_,slns1),(_,slns2)|
         slns2.iter().map(|wr|wr.word.len()).max().unwrap().cmp(
            &slns1.iter().map(|wr|wr.word.len()).max().unwrap()
         )
    
    );
    best_boards.into_iter().take(quantity).collect()
}