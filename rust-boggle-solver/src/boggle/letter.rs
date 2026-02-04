use rand::prelude::*;

pub fn random_letter() -> char {
    let mut rng = rand::rng();

    rng.sample(rand::distr::Alphabetic) as char
}

const SCRABBLE_LETTERS: [char;98] = [
  'e','e','e','e','e','e','e','e','e','e','e','e',
  'a','a','a','a','a','a','a','a','a',
  'i','i','i','i','i','i','i','i','i',
  'o','o','o','o','o','o','o','o',
  'n','n','n','n','n','n',
  'r','r','r','r','r','r',
  't','t','t','t','t','t',
  'l','l','l','l',
  's','s','s','s',
  'u','u','u','u',
  'd','d','d','d',
  'g','g','g',
  'b','b','c','c','m','m','p','p',
  'f','f','h','h','v','v','w','w','y','y',
  'k',
  'j','x',
  'q','z'
];

pub fn random_scrabble_letter() -> char {
    let mut rng = rand::rng();

    let index = rng.random_range(0..SCRABBLE_LETTERS.len());

    SCRABBLE_LETTERS[index]
}