use std::collections::HashMap;

use crate::{WORDS_ONE_LETTER_DICT, WORDS_TWO_LETTER_DICT};

const MIN_LENGTH: usize = 3;
const LETTERS_COUNT: usize = 7;
const MAX_LENGTH: usize = 10;

pub trait SpellingBee {
    fn scan_dict(&self) -> Vec<String>;
}

#[derive(Debug)]
pub struct SpellingBeeSimpleParams {
    letters: Vec<u8>,
    required_letter: u8,
}

impl SpellingBeeSimpleParams {
    /// First letter is the required one.
    /// Count of letters must be equal to `LETTERS_COUNT`.
    pub fn new(letters: &str) -> Self {
        let bytes = letters.as_bytes();
        assert!(
            bytes.len() == LETTERS_COUNT,
            "Letters count must be {}",
            LETTERS_COUNT
        );
        Self {
            letters: bytes.to_vec(),
            required_letter: bytes[0],
        }
    }
}

impl SpellingBee for SpellingBeeSimpleParams {
    fn scan_dict(&self) -> Vec<String> {
        self.letters
            .iter()
            .fold(Vec::new(), |mut res: Vec<String>, start_letter| {
                if let Some(words) = WORDS_ONE_LETTER_DICT.get(start_letter) {
                    'word_loop: for word in words.iter() {
                        if word.len() <= MIN_LENGTH || word.len() > MAX_LENGTH {
                            continue;
                        }
                        let mut contains_required = false;
                        for letter in word.iter() {
                            if !self.letters.contains(letter) {
                                continue 'word_loop;
                            }
                            contains_required =
                                contains_required || self.required_letter == *letter;
                        }
                        if contains_required {
                            res.push(String::from_utf8_lossy(word).to_string());
                        }
                    }
                }
                res
            })
    }
}

#[derive(Debug)]
pub struct SpellingBeeHintedParams {
    letters: Vec<u8>,
    required_letter: u8,
    letters_len: HashMap<u8, Vec<usize>>,
    start_letters: Vec<[u8; 2]>,
}

impl SpellingBeeHintedParams {
    pub fn new(
        letters: &str,
        letters_len: Vec<(u8, Vec<usize>)>,
        start_letters: Vec<[u8; 2]>,
    ) -> Self {
        let bytes = letters.as_bytes();
        assert!(
            bytes.len() == LETTERS_COUNT,
            "Letters count must be {}",
            LETTERS_COUNT
        );
        Self {
            letters: bytes.to_vec(),
            required_letter: bytes[0],
            letters_len: letters_len.into_iter().collect(),
            start_letters,
        }
    }
}

impl SpellingBeeHintedParams {
    fn scan_words(&self, res: &mut Vec<String>, words: &&[&[u8]], words_len: Option<&Vec<usize>>) {
        'word_loop: for word in words.iter() {
            if !self.letters_len.is_empty()
                && words_len.is_some_and(|acceptable_len| !acceptable_len.contains(&word.len()))
            {
                continue;
            }
            if word.len() <= MIN_LENGTH || word.len() > MAX_LENGTH {
                continue;
            }
            let mut contains_required = false;
            for letter in word.iter() {
                if !self.letters.contains(letter) {
                    continue 'word_loop;
                }
                contains_required = contains_required || self.required_letter == *letter;
            }
            if contains_required {
                res.push(String::from_utf8_lossy(word).to_string());
            }
        }
    }
}

impl SpellingBee for SpellingBeeHintedParams {
    fn scan_dict(&self) -> Vec<String> {
        if !self.start_letters.is_empty() {
            self.start_letters
                .iter()
                .fold(Vec::new(), |mut res: Vec<String>, start_letters| {
                    if let Some(words) = WORDS_TWO_LETTER_DICT.get(start_letters) {
                        let words_len = start_letters
                            .first()
                            .and_then(|start_letter| self.letters_len.get(start_letter));
                        if !self.letters_len.is_empty() && words_len.is_none() {
                            return res;
                        }
                        self.scan_words(&mut res, words, words_len);
                    }
                    res
                })
        } else {
            self.letters
                .iter()
                .fold(Vec::new(), |mut res: Vec<String>, start_letter| {
                    if let Some(words) = WORDS_ONE_LETTER_DICT.get(start_letter) {
                        let words_len = self.letters_len.get(start_letter);
                        if !self.letters_len.is_empty() && words_len.is_none() {
                            return res;
                        }
                        self.scan_words(&mut res, words, words_len);
                    }
                    res
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_with_simple() {
        let game = SpellingBeeSimpleParams::new("zwieslt");
        let words = game.scan_dict();
        assert_eq!(words.len(), 51);
    }

    #[test]
    fn it_finds_with_hinted_simple() {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![]);
        let words = game.scan_dict();
        assert_eq!(words.len(), 146);
    }

    #[test]
    fn it_finds_with_hinted_with_length() {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![]);
        let words = game.scan_dict();
        assert_eq!(
            words.len(),
            words
                .iter()
                .filter(|w| w.starts_with('a') && w.len() == 4)
                .count()
        );
        assert_eq!(words.len(), 16);
    }

    #[test]
    fn it_finds_with_hinted_with_starting() {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![[b'a', b'c']]);
        let words = game.scan_dict();
        assert_eq!(
            words.len(),
            words
                .iter()
                .filter(|w| w.starts_with('a') || w.starts_with('c'))
                .count()
        );
        assert_eq!(words.len(), 6);
    }

    #[test]
    fn it_finds_with_hinted_with_starting_and_length() {
        let game =
            SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![[b'a', b'b']]);
        let words = game.scan_dict();
        assert_eq!(
            words.len(),
            words
                .iter()
                .filter(|w| w.starts_with('a') && w.len() == 4)
                .count()
        );
        assert_eq!(words.len(), 4);
    }
}
