#![allow(dead_code)]
use crate::trie::Trie;

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

    pub fn search(&self, _trie: &Trie) -> Vec<String> {
        vec![]
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
        // let trie = make_trie();

        // let game = LetterBoxed::new("abcdefg");
        // let mut words = game.scan(&trie);
        // words.sort();
        // assert_eq!(words, vec!["abcdefg", "abce", "acce"]);
    }
}
