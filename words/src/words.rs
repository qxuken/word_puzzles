use crate::words_file::FILE;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type WordsDict = Vec<&'static [u8]>;
pub type WordsOneLetterDict = HashMap<u8, &'static [&'static [u8]]>;
pub type WordsTwoLetterDict = HashMap<[u8; 2], &'static [&'static [u8]]>;

pub static WORDS: Lazy<WordsDict> = Lazy::new(|| {
    let mut vec = Vec::with_capacity(370_105);
    FILE.split(|&byte| byte == b'\n')
        .filter(|w| !w.is_empty())
        .for_each(|word| {
            vec.push(word);
        });
    vec.sort();
    vec
});

pub static WORDS_ONE_LETTER_DICT: Lazy<WordsOneLetterDict> = Lazy::new(|| {
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
    map.insert(*letter, &WORDS[start_index..index]);
    map
});

pub static WORDS_TWO_LETTER_DICT: Lazy<WordsTwoLetterDict> = Lazy::new(|| {
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
                let w = WORDS.get(i + index + 1).unwrap();
                letters = [w[0], w[1]];
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
    map.insert(letters, &WORDS[start_index..index]);
    map.shrink_to_fit();
    map
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
    fn it_loads_words_one_letter_dict() {
        let letter = b'z';
        let words_from_dict: Vec<String> = WORDS_ONE_LETTER_DICT
            .get(&letter)
            .unwrap()
            .iter()
            .map(|w| String::from_utf8_lossy(w).to_string())
            .collect();
        let words_from_words: Vec<String> = WORDS
            .iter()
            .filter(|w| w.starts_with(&[letter]))
            .map(|w| String::from_utf8_lossy(w).to_string())
            .collect();

        assert_eq!(words_from_dict.len(), words_from_words.len());
        assert_eq!(words_from_dict, words_from_words);
    }

    #[test]
    fn it_loads_words_two_letter_dict() {
        let letter = b"zy";
        let words_from_dict: Vec<String> = WORDS_TWO_LETTER_DICT
            .get(letter)
            .unwrap()
            .iter()
            .map(|w| String::from_utf8_lossy(w).to_string())
            .collect();
        let words_from_words: Vec<String> = WORDS
            .iter()
            .filter(|w| w.starts_with(letter))
            .map(|w| String::from_utf8_lossy(w).to_string())
            .collect();

        assert_eq!(words_from_dict.len(), words_from_words.len());
        assert_eq!(words_from_dict, words_from_words);
    }
}
