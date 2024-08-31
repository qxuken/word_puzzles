use crate::{WordsDict, WordsShortcuts};
use anyhow::{bail, Result};
use std::collections::HashMap;

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 10;
pub const LETTERS_COUNT: usize = 7;

pub trait SpellingBee {
    fn scan_dict(&self, dict: &WordsDict, shortcuts: &WordsShortcuts) -> Vec<String>;
}

#[derive(Debug)]
pub struct SpellingBeeSimpleParams {
    letters: [u8; LETTERS_COUNT],
    required_letter: u8,
}

impl SpellingBeeSimpleParams {
    /// First letter is the required one.
    /// Count of letters must be equal to `LETTERS_COUNT`.
    pub fn new(letters: &str) -> Result<Self> {
        let mut bytes: Vec<u8> = letters.as_bytes().into();
        bytes.dedup();
        if bytes.len() != LETTERS_COUNT {
            bail!("letters must have {} unique characters", LETTERS_COUNT);
        }
        let required_letter = *bytes.first().unwrap();
        let letters = bytes.try_into().unwrap();
        Ok(Self {
            letters,
            required_letter,
        })
    }
}

impl SpellingBee for SpellingBeeSimpleParams {
    fn scan_dict(&self, dict: &WordsDict, shortcuts: &WordsShortcuts) -> Vec<String> {
        self.letters
            .iter()
            .fold(Vec::new(), |mut res: Vec<String>, start_letter| {
                let Some(range) = shortcuts.search_range(&[*start_letter]) else {
                    return res;
                };
                'word_loop: for word in dict.iter_range(range) {
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
                res
            })
    }
}

#[derive(Debug)]
pub struct SpellingBeeHintedParams {
    letters: [u8; LETTERS_COUNT],
    required_letter: u8,
    letters_len: HashMap<u8, Vec<usize>>,
    start_letters: Vec<[u8; 2]>,
}

impl SpellingBeeHintedParams {
    pub fn new(
        letters: &str,
        letters_len: Vec<(u8, Vec<usize>)>,
        start_letters: Vec<[u8; 2]>,
    ) -> Result<Self> {
        let mut bytes: Vec<u8> = letters.as_bytes().into();
        bytes.dedup();
        if bytes.len() != LETTERS_COUNT {
            bail!("letters must have {} unique characters", LETTERS_COUNT);
        }
        let required_letter = *bytes.first().unwrap();
        let letters = bytes.try_into().unwrap();
        Ok(Self {
            letters,
            required_letter,
            letters_len: letters_len.into_iter().collect(),
            start_letters,
        })
    }
}

impl<'a> SpellingBeeHintedParams {
    fn scan_words(
        &self,
        res: &mut Vec<String>,
        words: impl Iterator<Item = &'a &'a [u8]>,
        words_len: Option<&Vec<usize>>,
    ) {
        'word_loop: for word in words {
            if !self.letters_len.is_empty()
                && words_len.is_some_and(|acceptable_len| !acceptable_len.contains(&word.len()))
            {
                continue;
            }
            if MIN_LENGTH >= word.len() || word.len() > MAX_LENGTH {
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
    fn scan_dict(&self, dict: &WordsDict, shortcuts: &WordsShortcuts) -> Vec<String> {
        if !self.start_letters.is_empty() {
            self.start_letters
                .iter()
                .fold(Vec::new(), |mut res: Vec<String>, start_letters| {
                    let Some(range) = shortcuts.search_range(start_letters) else {
                        return res;
                    };
                    let words = dict.iter_range(range);
                    let words_len = start_letters
                        .first()
                        .and_then(|start_letter| self.letters_len.get(start_letter));
                    if !self.letters_len.is_empty() && words_len.is_none() {
                        return res;
                    }
                    self.scan_words(&mut res, words, words_len);
                    res
                })
        } else {
            self.letters
                .iter()
                .fold(Vec::new(), |mut res: Vec<String>, start_letter| {
                    let Some(range) = shortcuts.search_range(&[*start_letter]) else {
                        return res;
                    };
                    let words = dict.iter_range(range);
                    let words_len = self.letters_len.get(start_letter);
                    if !self.letters_len.is_empty() && words_len.is_none() {
                        return res;
                    }
                    self.scan_words(&mut res, words, words_len);
                    res
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use test::Bencher;

    static DICT: LazyLock<WordsDict> = LazyLock::new(WordsDict::load);

    static SHORTCUTS: LazyLock<WordsShortcuts> = LazyLock::new(|| WordsShortcuts::new(&DICT));

    #[test]
    fn it_finds_with_simple() {
        let game = SpellingBeeSimpleParams::new("zwieslt").unwrap();
        let words = game.scan_dict(&DICT, &SHORTCUTS);
        assert_eq!(words.len(), 51);
    }

    #[bench]
    fn bench_it_finds_with_simple(b: &mut Bencher) {
        let game = SpellingBeeSimpleParams::new("zwieslt").unwrap();
        b.iter(|| game.scan_dict(&DICT, &SHORTCUTS));
    }

    #[test]
    fn it_finds_with_hinted_simple() {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![]).unwrap();
        let words = game.scan_dict(&DICT, &SHORTCUTS);
        assert_eq!(words.len(), 146);
    }

    #[bench]
    fn bench_it_finds_with_hinted_simple(b: &mut Bencher) {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![]).unwrap();
        b.iter(|| game.scan_dict(&DICT, &SHORTCUTS));
    }

    #[test]
    fn it_finds_with_hinted_with_length() {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![]).unwrap();
        let words = game.scan_dict(&DICT, &SHORTCUTS);
        assert_eq!(
            words.len(),
            words
                .iter()
                .filter(|w| w.starts_with('a') && w.len() == 4)
                .count()
        );
        assert_eq!(words.len(), 16);
    }

    #[bench]
    fn bench_it_finds_with_hinted_with_length(b: &mut Bencher) {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![]).unwrap();
        b.iter(|| game.scan_dict(&DICT, &SHORTCUTS));
    }

    #[test]
    fn it_finds_with_hinted_with_starting() {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![[b'a', b'c']]).unwrap();
        let words = game.scan_dict(&DICT, &SHORTCUTS);
        assert_eq!(
            words.len(),
            words
                .iter()
                .filter(|w| w.starts_with('a') || w.starts_with('c'))
                .count()
        );
        assert_eq!(words.len(), 6);
    }

    #[bench]
    fn bench_it_finds_with_hinted_with_starting(b: &mut Bencher) {
        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![[b'a', b'c']]).unwrap();
        b.iter(|| game.scan_dict(&DICT, &SHORTCUTS));
    }

    #[test]
    fn it_finds_with_hinted_with_starting_and_length() {
        let game =
            SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![[b'a', b'b']])
                .unwrap();
        let words = game.scan_dict(&DICT, &SHORTCUTS);
        assert_eq!(
            words.len(),
            words
                .iter()
                .filter(|w| w.starts_with('a') && w.len() == 4)
                .count()
        );
        assert_eq!(words.len(), 4);
    }

    #[bench]
    fn bench_it_finds_with_hinted_with_starting_and_length(b: &mut Bencher) {
        let game =
            SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![[b'a', b'b']])
                .unwrap();
        b.iter(|| game.scan_dict(&DICT, &SHORTCUTS));
    }
}
