// #![feature(test)]
// run with ```rustup run nightly cargo bench```
// extern crate test;

use once_cell::sync::Lazy;
use trie::Trie;
pub mod games;
pub mod trie;

const FILE: &[u8; 3864811] = include_bytes!("../data/words_alpha.txt");

pub static WORDS: Lazy<Vec<&[u8]>> = Lazy::new(|| {
    let mut vec = Vec::with_capacity(370_105);
    FILE.split(|&byte| byte == b'\n').for_each(|word| {
        vec.push(word);
    });
    vec
});

pub static TRIE: Lazy<Trie> = Lazy::new(|| {
    let mut trie = Trie::new();
    FILE.split(|&byte| byte == b'\n').for_each(|word| {
        trie.append_bytes(word);
    });
    trie
});

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

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

        assert!(TRIE.search("abc"));
    }

    // #[bench]
    // fn it_searches_words_starts_with_a(b: &mut Bencher) {
    //     let _load = WORDS.get(0).unwrap();
    //     b.iter(|| {
    //         WORDS
    //             .iter()
    //             .filter(|word| word.starts_with(b"a"))
    //             .map(|word| String::from_utf8_lossy(&word).to_string())
    //             .collect::<Vec<String>>()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_starts_with_ab(b: &mut Bencher) {
    //     let _load = WORDS.get(0).unwrap();
    //     b.iter(|| {
    //         WORDS
    //             .iter()
    //             .filter(|word| word.starts_with(b"ab"))
    //             .map(|word| String::from_utf8_lossy(&word).to_string())
    //             .collect::<Vec<String>>()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_starts_with_abc(b: &mut Bencher) {
    //     let _load = WORDS.get(0).unwrap();
    //     b.iter(|| {
    //         WORDS
    //             .iter()
    //             .filter(|word| word.starts_with(b"abc"))
    //             .map(|word| String::from_utf8_lossy(&word).to_string())
    //             .collect::<Vec<String>>()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_starts_with_hip(b: &mut Bencher) {
    //     let _load = WORDS.get(0).unwrap();
    //     b.iter(|| {
    //         WORDS
    //             .iter()
    //             .filter(|word| word.starts_with(b"hip"))
    //             .map(|word| String::from_utf8_lossy(&word).to_string())
    //             .collect::<Vec<String>>()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_starts_with_tan(b: &mut Bencher) {
    //     let _load = WORDS.get(0).unwrap();
    //     b.iter(|| {
    //         WORDS
    //             .iter()
    //             .filter(|word| word.starts_with(b"tan"))
    //             .map(|word| String::from_utf8_lossy(&word).to_string())
    //             .collect::<Vec<String>>()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_starts_with_z(b: &mut Bencher) {
    //     let _load = WORDS.get(0).unwrap();
    //     b.iter(|| {
    //         WORDS
    //             .iter()
    //             .filter(|word| word.starts_with(b"z"))
    //             .map(|word| String::from_utf8_lossy(&word).to_string())
    //             .collect::<Vec<String>>()
    //     });
    // }

    // #[bench]
    // fn it_searches_trie_starts_with_a(b: &mut Bencher) {
    //     let _load = TRIE.is_word;
    //     b.iter(|| TRIE.starts_with("a"));
    // }

    // #[bench]
    // fn it_searches_trie_starts_with_abc(b: &mut Bencher) {
    //     let _load = TRIE.is_word;
    //     b.iter(|| TRIE.starts_with("abc"));
    // }

    // #[bench]
    // fn it_searches_trie_starts_with_ab(b: &mut Bencher) {
    //     let _load = TRIE.is_word;
    //     b.iter(|| TRIE.starts_with("ab"));
    // }

    // #[bench]
    // fn it_searches_trie_starts_with_hip(b: &mut Bencher) {
    //     let _load = TRIE.is_word;
    //     b.iter(|| TRIE.starts_with("hip"));
    // }

    // #[bench]
    // fn it_searches_trie_starts_with_tan(b: &mut Bencher) {
    //     let _load = TRIE.is_word;
    //     b.iter(|| TRIE.starts_with("tan"));
    // }

    // #[bench]
    // fn it_searches_trie_starts_with_z(b: &mut Bencher) {
    //     let _load = TRIE.is_word;
    //     b.iter(|| TRIE.starts_with("z"));
    // }
}
