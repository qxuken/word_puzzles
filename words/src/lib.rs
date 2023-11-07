use std::{borrow::Cow, cell::RefCell, ops::Deref};

use once_cell::sync::Lazy;
use word::Word;
use words::Words;
mod word;
mod words;

const NEW_LINE: &u8 = &b'\n';

const FILE: &[u8; 3864811] = include_bytes!("../data/words_alpha.txt");

// const WORDS_DICT: &[Cow<'static, str>; 370105] = {
//     let words: Vec<Cow<'static, str>> = FILE
//         .split(|v| v == NEW_LINE)
//         .map(|w| String::from_utf8_lossy(w))
//         .collect();
//     &words[..]
// };

pub static WORDS: Lazy<Words> = Lazy::new(|| Words::load_words_dict(FILE));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_words() {
        assert_eq!(WORDS.get(0), Some(&"a".into()));
        assert!(WORDS.len() > 350_000);
    }
}
