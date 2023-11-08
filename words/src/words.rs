use crate::word::Word;

const NEW_LINE: &u8 = &b'\n';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Words(Vec<Word>);

impl Words {
    pub fn new(v: Vec<Word>) -> Words {
        Words(v)
    }

    pub fn get(&self, i: usize) -> Option<&Word> {
        self.0.get(i)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn load_words_dict(words: &'static [u8]) -> Words {
        words
            .split(|v| v == NEW_LINE)
            .map(|w| String::from_utf8_lossy(w).into())
            .collect::<Vec<Word>>()
            .into()
    }
}

impl From<Vec<Word>> for Words {
    fn from(value: Vec<Word>) -> Self {
        Words::new(value)
    }
}
