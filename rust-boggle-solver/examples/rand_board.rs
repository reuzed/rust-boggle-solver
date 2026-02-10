use rust_boggle_solver::{board::random_board, solver::solve};

fn main() {
    let board = random_board();

    println!("{}", board);
    for wr in solve(&board) {
        print!("{}, ", wr.word)
    }
    println!("{}", board);

    let start = std::time::Instant::now();
    for _ in 0..100 {
        let board = random_board();

        println!("{}", board);
        for wr in solve(&board) {
            print!("{}, ", wr.word)
        }
        println!("{}", board);
    }
    println!("{:?}", start.elapsed());
}
