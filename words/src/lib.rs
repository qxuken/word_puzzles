// #![feature(test)]
// run with ```rustup run nightly cargo bench```
// extern crate test;

use std::collections::HashMap;

use once_cell::sync::Lazy;
use trie::Trie;
pub mod games;
pub mod trie;

const FILE: &[u8; 3864811] = include_bytes!("../data/words_alpha.txt");

pub static WORDS: Lazy<Vec<&[u8]>> = Lazy::new(|| {
    let mut vec = Vec::with_capacity(370_105);
    FILE.split(|&byte| byte == b'\n')
        .filter(|w| !w.is_empty())
        .for_each(|word| {
            vec.push(word);
        });
    vec
});

pub static WORDS_ONE_LETTER_DICT: Lazy<HashMap<u8, &[&[u8]]>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(26);
    if WORDS.is_empty() {
        return map;
    }
    let mut start_index = 0;
    let mut index = 0;
    let mut letter = WORDS.first().unwrap().first().unwrap();
    while let Some(word) = WORDS.get(index) {
        if let Some(word_letter) = word.first() {
            if word_letter != letter {
                map.insert(*letter, &WORDS[start_index..index]);
                letter = word_letter;
                start_index = index;
            }
        }
        index += 1;
    }
    map
});

pub static WORDS_TWO_LETTER_DICT: Lazy<HashMap<[u8; 2], &[&[u8]]>> = Lazy::new(|| {
    let mut map = HashMap::new();
    if WORDS.is_empty() {
        return map;
    }
    let mut start_index = WORDS.iter().position(|w| w.len() == 2).unwrap();
    let mut index = start_index;
    let first_word = WORDS.get(start_index).unwrap();
    let mut letters = [first_word[0], first_word[1]];
    while let Some(word) = WORDS.get(index) {
        if word.len() == 1 {
            map.insert(letters, &WORDS[start_index..index]);
            if let Some(i) = WORDS.iter().skip(index + 1).position(|w| w.len() == 2) {
                start_index = i + index + 1;
                index = i + index + 1;
                continue;
            } else {
                break;
            }
        }
        if !word.starts_with(&letters) {
            map.insert(letters, &WORDS[start_index..index]);
            letters = [word[0], word[1]];
            start_index = index;
        }
        index += 1;
    }
    map.shrink_to_fit();
    map
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
    // use test::Bencher;
    use super::*;

    #[test]
    fn it_loads_words() {
        assert_eq!(WORDS.get(0), Some(&"a".as_bytes()));
        assert!(WORDS.len() == 370_105);
    }

    #[test]
    fn it_loads_words_one_letter_dict() {
        let letter = b'a';
        let words = WORDS_ONE_LETTER_DICT.get(&letter).unwrap();
        assert_eq!(words.first().and_then(|w| w.first()), Some(&letter));
        assert_eq!(words.last().and_then(|w| w.first()), Some(&letter));
    }

    #[test]
    fn it_loads_words_two_letter_dict() {
        let letter = b"ac";
        let words = WORDS_TWO_LETTER_DICT.get(letter).unwrap();

        assert!(words.first().filter(|w| w.starts_with(letter)).is_some());
        assert!(words.last().filter(|w| w.starts_with(letter)).is_some());
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
    // fn it_searches_words_dicts_starts_with_a(b: &mut Bencher) {
    //     let _load = WORDS_ONE_LETTER_DICT.len();
    //     b.iter(|| {
    //         WORDS_ONE_LETTER_DICT
    //             .get(&b'a')
    //             .map(|words| {
    //                 words
    //                     .iter()
    //                     .map(|word| String::from_utf8_lossy(&word).to_string())
    //                     .collect::<Vec<String>>()
    //             })
    //             .unwrap_or_default()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_dicts_starts_with_ab(b: &mut Bencher) {
    //     let _load = WORDS_TWO_LETTER_DICT.len();
    //     b.iter(|| {
    //         WORDS_TWO_LETTER_DICT
    //             .get(b"ab")
    //             .map(|words| {
    //                 words
    //                     .iter()
    //                     .map(|word| String::from_utf8_lossy(&word).to_string())
    //                     .collect::<Vec<String>>()
    //             })
    //             .unwrap_or_default()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_dicts_starts_with_abc(b: &mut Bencher) {
    //     let _load = WORDS_TWO_LETTER_DICT.len();
    //     b.iter(|| {
    //         WORDS_TWO_LETTER_DICT
    //             .get(b"ab")
    //             .map(|words| {
    //                 words
    //                     .iter()
    //                     .filter(|word| word.starts_with(b"abc"))
    //                     .map(|word| String::from_utf8_lossy(&word).to_string())
    //                     .collect::<Vec<String>>()
    //             })
    //             .unwrap_or_default()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_dicts_starts_with_hip(b: &mut Bencher) {
    //     let _load = WORDS_TWO_LETTER_DICT.len();
    //     b.iter(|| {
    //         WORDS_TWO_LETTER_DICT
    //             .get(b"hi")
    //             .map(|words| {
    //                 words
    //                     .iter()
    //                     .filter(|word| word.starts_with(b"hip"))
    //                     .map(|word| String::from_utf8_lossy(&word).to_string())
    //                     .collect::<Vec<String>>()
    //             })
    //             .unwrap_or_default()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_dicts_starts_with_tan(b: &mut Bencher) {
    //     let _load = WORDS_TWO_LETTER_DICT.len();
    //     b.iter(|| {
    //         WORDS_TWO_LETTER_DICT
    //             .get(b"ta")
    //             .map(|words| {
    //                 words
    //                     .iter()
    //                     .filter(|word| word.starts_with(b"tan"))
    //                     .map(|word| String::from_utf8_lossy(&word).to_string())
    //                     .collect::<Vec<String>>()
    //             })
    //             .unwrap_or_default()
    //     });
    // }

    // #[bench]
    // fn it_searches_words_dicts_starts_with_z(b: &mut Bencher) {
    //     let _load = WORDS_ONE_LETTER_DICT.len();
    //     b.iter(|| {
    //         WORDS_ONE_LETTER_DICT
    //             .get(&b'z')
    //             .map(|words| {
    //                 words
    //                     .iter()
    //                     .map(|word| String::from_utf8_lossy(&word).to_string())
    //                     .collect::<Vec<String>>()
    //             })
    //             .unwrap_or_default()
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
