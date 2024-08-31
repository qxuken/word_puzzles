use std::ops::Range;

pub const FILE: &[u8; 3_864_811] = include_bytes!("../data/words_alpha.txt");

pub struct WordsDict {
    words: Vec<&'static [u8]>,
}

impl WordsDict {
    pub fn load() -> Self {
        let mut words: Vec<_> = FILE
            .split(|&byte| byte == b'\n')
            .filter(|w| !w.is_empty())
            .collect();
        words.sort();

        Self { words }
    }
}

impl WordsDict {
    pub fn get(&self, at: usize) -> Option<&&[u8]> {
        self.words.get(at)
    }

    pub fn iter_range(&self, range: Range<usize>) -> impl Iterator<Item = &&[u8]> {
        self.words[range].iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &&[u8]> {
        self.words.iter()
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }
}
