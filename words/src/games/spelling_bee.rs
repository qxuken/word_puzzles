use std::collections::HashMap;

use crate::trie::Trie;

const MIN_LENGTH: usize = 3;
const LETTERS_COUNT: usize = 7;
const MAX_LENGTH: usize = 10;

pub trait SpellingBee {
    fn scan(&self, _trie: &Trie) -> Vec<String>;
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

    fn dfs(&self, node: &Trie, words: &mut Vec<String>, path: &[u8]) {
        if path.len() > MAX_LENGTH {
            return;
        }
        if path.len() > MIN_LENGTH && node.is_word && path.contains(&self.required_letter) {
            words.push(String::from_utf8_lossy(path).to_string());
        }
        for (letter, child) in &node.children {
            if !self.letters.contains(letter) {
                continue;
            }
            let mut new_path = path.to_vec();
            new_path.push(*letter);
            self.dfs(child, words, &new_path);
        }
    }
}

impl SpellingBee for SpellingBeeSimpleParams {
    fn scan(&self, trie: &Trie) -> Vec<String> {
        let mut words = vec![];
        self.dfs(trie, &mut words, &[]);
        words
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

    fn dfs(&self, node: &Trie, words: &mut Vec<String>, path: &[u8]) {
        if path.len() > MAX_LENGTH {
            return;
        }
        if path.len() > MIN_LENGTH && node.is_word && path.contains(&self.required_letter) {
            if let Some(lengs) = self.letters_len.get(path.first().unwrap()) {
                if lengs.contains(&path.len()) {
                    words.push(String::from_utf8_lossy(path).to_string());
                }
            } else {
                words.push(String::from_utf8_lossy(path).to_string());
            }
        }
        for (letter, child) in &node.children {
            if !self.letters.contains(letter) {
                continue;
            }
            let mut new_path = path.to_vec();
            new_path.push(*letter);
            self.dfs(child, words, &new_path);
        }
    }
}

impl SpellingBee for SpellingBeeHintedParams {
    fn scan(&self, trie: &Trie) -> Vec<String> {
        let mut words = vec![];
        if self.start_letters.is_empty() {
            self.dfs(trie, &mut words, &[]);
        } else {
            for letters in &self.start_letters {
                if let Some(node) = trie.follow(letters) {
                    self.dfs(node, &mut words, letters);
                }
            }
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_trie() -> Trie {
        let mut trie = Trie::new();
        trie.append("ab");
        trie.append("abc");
        trie.append("abce");
        trie.append("acce");
        trie.append("abcdefg");
        trie.append("cbcdefg");
        trie.append("bbcdefg");
        trie.append("abcdefghijklmnopqrstuvwxyz");
        trie
    }

    #[test]
    fn it_finds_with_simple() {
        let trie = make_trie();

        let game = SpellingBeeSimpleParams::new("abcdefg");
        let mut words = game.scan(&trie);
        words.sort();
        assert_eq!(words, vec!["abcdefg", "abce", "acce"]);
    }

    #[test]
    fn it_finds_with_hinted_simple() {
        let trie = make_trie();

        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![]);
        let mut words = game.scan(&trie);
        words.sort();
        assert_eq!(words, vec!["abcdefg", "abce", "acce"]);
    }

    #[test]
    fn it_finds_with_hinted_with_length() {
        let trie = make_trie();

        let game = SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![]);
        let mut words = game.scan(&trie);
        words.sort();
        assert_eq!(words, vec!["abce", "acce"]);
    }

    #[test]
    fn it_finds_with_hinted_with_starting() {
        let trie = make_trie();

        let game = SpellingBeeHintedParams::new("abcdefg", vec![], vec![[b'a', b'c']]);
        let mut words = game.scan(&trie);
        words.sort();
        assert_eq!(words, vec!["acce"]);
    }

    #[test]
    fn it_finds_with_hinted_with_starting_and_length() {
        let trie = make_trie();

        let game =
            SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![7])], vec![[b'a', b'b']]);
        let mut words = game.scan(&trie);
        words.sort();
        assert_eq!(words, vec!["abcdefg"]);
    }
}
