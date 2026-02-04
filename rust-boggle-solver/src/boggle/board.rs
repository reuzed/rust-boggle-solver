use std::fmt;

use super::letter::random_scrabble_letter;

#[derive(Debug)]
pub struct Board {
    arr: [[char; 4]; 4]
}

impl Board {
    pub fn new(r1: [char;4], r2: [char;4], r3: [char;4], r4: [char;4]) -> Board {
        Board { arr: [r1,r2,r3,r4] }
    }

    pub fn at(&self, x:usize, y:usize) -> char {
        self.arr[y][x]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌─┬─┬─┬─┐")?;
        for (i, row) in self.arr.iter().enumerate(){
            writeln!(f, "│{}│{}│{}│{}│", row[0], row[1], row[2], row[3])?;
            if i != 3 {
                writeln!(f, "├─┼─┼─┼─┤")?;
            }  
        }
        write!(f, "└─┴─┴─┴─┘")
    }
}

const ADJACENCIES: [[i32; 2]; 8] = [
    [1,-1],
    [1,0],
    [1,1],
    [0,-1],
    [0,1],
    [-1,-1],
    [-1,0],
    [-1,1],
];

fn neighbouring_indices(x: i32, y: i32) -> Vec<[i32;2]> {
    // Return the indices adjacent and within grid
    ADJACENCIES.iter().map(|p| [p[0] + x, p[1] + y] )
    .filter(|p| 0 <= p[0] && p[0] < 4 && 0 <= p[1] && p[1] <= 4)
    .collect()
}

fn random_row() -> [char; 4] {
    [random_scrabble_letter(), random_scrabble_letter(), random_scrabble_letter(), random_scrabble_letter()]
}
pub fn random_board() -> Board {
    Board::new(random_row(), random_row(), random_row(), random_row())
}