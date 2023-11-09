use once_cell::sync::Lazy;
use trie::Trie;
pub mod games;
pub mod trie;

const FILE: &[u8; 3864811] = include_bytes!("../data/words_alpha.txt");

pub static WORDS: Lazy<Vec<&[u8]>> =
    Lazy::new(|| FILE.split(|&byte| byte == b'\n').collect::<Vec<&[u8]>>());

pub static TRIE: Lazy<Trie> = Lazy::new(|| {
    let mut trie = Trie::new();
    for &word in WORDS.iter() {
        trie.append_bytes(word);
    }
    trie
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_words() {
        assert_eq!(WORDS.get(0), Some(&"a".as_bytes()));
        assert!(WORDS.len() == 370_105);
    }

    #[test]
    fn it_loads_trie() {
        let mut words: Vec<String> = vec![];

        TRIE.dfs(&mut words, &[]);
        assert!(words.len() == 370_105);

        assert!(TRIE.search("a"));
    }
}
