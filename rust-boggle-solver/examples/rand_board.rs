use rust_boggle_solver::{board::random_board, solver::solve};

fn main() {
    for i in 0..1{

        let board = random_board();
    
        println!("{}", board);
        for wr in solve(&board) {
            print!("{}, ", wr.word)
        }
        println!("{}", board);
    }
}