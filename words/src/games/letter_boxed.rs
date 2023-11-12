#![allow(dead_code)]

#[derive(Debug)]
pub struct LetterBoxed {
    top: [u8; 3],
    right: [u8; 3],
    bottom: [u8; 3],
    left: [u8; 3],
}

impl LetterBoxed {
    pub fn new(top: [u8; 3], right: [u8; 3], bottom: [u8; 3], left: [u8; 3]) -> LetterBoxed {
        LetterBoxed {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn search(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_with_simple() {
        let game = LetterBoxed::new(*b"abc", *b"def", *b"ghi", *b"jkl");
        let mut words: Vec<String> = game.search();
        words.sort();
        let expected: Vec<String> = vec![];
        assert_eq!(words, expected);
    }
}
