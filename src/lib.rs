use std::char;
use std::iter::Iterator;

use rand;
use rand::seq::SliceRandom;

pub fn randstr(choices: &[char], len: usize) -> String {
    let mut rng = rand::thread_rng();

    let mut randstr = String::new();
    for _ in 0..len {
        randstr.push(*choices.choose(&mut rng).unwrap())
    }
    randstr
}

pub fn ascii_lowercase_chars() -> Vec<char> {
    ('a' as u32 ..= 'z' as u32).filter_map(char::from_u32).collect()
}

pub fn ascii_uppercase_chars() -> Vec<char> {
    ascii_lowercase_chars().iter().map(char::to_ascii_uppercase).collect()
}

pub fn ascii_numeric_chars() -> Vec<char> {
    ('0' as u32 ..= '9' as u32).filter_map(char::from_u32).collect()
}