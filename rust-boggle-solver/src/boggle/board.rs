use std::fmt;

use super::letter::random_scrabble_letter;

pub struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Board {
    arr: [[char; 4]; 4]
}

impl Board {
    pub fn new(r1: [char;4], r2: [char;4], r3: [char;4], r4: [char;4]) -> Board {
        Board { arr: [r1,r2,r3,r4] }
    }

    pub fn at(&self, c: Coord) -> char {
        self.arr[c.y][c.x]
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

pub fn coords_vec() -> Vec<Coord>{
    // Get a vector of all coordinates in the grid
    (0..4).map(
        |x| (0..4).map(
            move |y| Coord{x,y}
        )
    ).flatten().collect()
}

pub fn neighbouring_coords(c: Coord) -> Vec<Coord> {
    // Return the indices adjacent and within grid
    let (x,y) = (c.x as i32, c.y as i32);
    ADJACENCIES.iter().map(|p| [p[0] + x, p[1] + y] )
    .filter(|p| 0 <= p[0] && p[0] < 4 && 0 <= p[1] && p[1] <= 4)
    .map(|p| Coord { x: p[0] as usize, y: p[1] as usize })
    .collect()
}

fn random_row() -> [char; 4] {
    [random_scrabble_letter(), random_scrabble_letter(), random_scrabble_letter(), random_scrabble_letter()]
}
pub fn random_board() -> Board {
    Board::new(random_row(), random_row(), random_row(), random_row())
}