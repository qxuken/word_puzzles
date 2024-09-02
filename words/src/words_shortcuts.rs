use std::ops::Range;

use crate::words_dict::WordsDict;

const DEPTH: usize = 2;

const fn calc_array_size(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    26u32.pow(n as u32) as usize + calc_array_size(n - 1)
}

const ARRAY_SIZE: usize = calc_array_size(DEPTH);

#[derive(Debug, Clone, Copy)]
pub struct WordsShortcuts {
    size: u32,
    shortcuts: [i32; ARRAY_SIZE],
}

impl WordsShortcuts {
    pub fn new(dict: &WordsDict) -> Self {
        let mut shortcuts = [-1; ARRAY_SIZE];
        shortcuts[0] = 0;

        let mut latest_first = *dict.get(0).unwrap().first().unwrap();
        let mut latest_second = None;

        for (i, word) in dict.iter().enumerate() {
            let first = word.first().unwrap().to_ascii_lowercase();
            if latest_first != first {
                shortcuts[Self::calc_one_leter_i(first)] = i as i32;
                latest_first = first;
            }
            let second = word.get(1).map(|c| c.to_ascii_lowercase());
            match (second, latest_second) {
                (None, _) => latest_second = None,
                (Some(curr), Some(latest)) if curr == latest => {}
                (Some(second), _) => {
                    shortcuts[Self::calc_two_leter_i(first, second)] = i as i32;
                    latest_second = Some(second);
                }
            }
        }

        Self {
            size: dict.size() as u32,
            shortcuts,
        }
    }
}

impl<'a> WordsShortcuts {
    pub fn search_range(&self, prefix: &'a [u8]) -> Option<Range<usize>> {
        let mut bytes = prefix.iter();

        let mut right = self.size as usize;

        let Some(first_char) = bytes.next().map(|c| c.to_ascii_lowercase()) else {
            return Some(0..right);
        };

        let left = self
            .shortcuts
            .get(Self::calc_one_leter_i(first_char))
            .filter(|&c| c >= &0)
            .copied()
            .map(|c| c as usize)?;

        for offset in (first_char + 1)..=122 {
            if let Some(next_seg) = self
                .shortcuts
                .get(Self::calc_one_leter_i(offset))
                .filter(|&c| c >= &0)
                .copied()
                .map(|c| c as usize)
            {
                right = right.min(next_seg);
                break;
            }
        }

        let Some(second_char) = bytes.next().map(|c| c.to_ascii_lowercase()) else {
            return Some(left..right);
        };

        let left = self
            .shortcuts
            .get(Self::calc_two_leter_i(first_char, second_char))
            .copied()
            .filter(|c| c > &0)
            .map(|c| c as usize)?;

        for offset in (second_char + 1)..=122 {
            if let Some(next_seg) = self
                .shortcuts
                .get(Self::calc_two_leter_i(first_char, offset))
                .filter(|&c| c >= &0)
                .copied()
                .map(|c| c as usize)
            {
                right = right.min(next_seg);
                break;
            }
        }

        Some(left..right)
    }
}

impl WordsShortcuts {
    fn calc_one_leter_i(one: u8) -> usize {
        (one as usize - 97) * 26
    }

    fn calc_two_leter_i(first: u8, second: u8) -> usize {
        Self::calc_one_leter_i(first) + second as usize - 96
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
    fn search_empty_letter() {
        let range = SHORTCUTS.search_range(b"");
        assert_eq!(range, Some(0..DICT.size()))
    }

    #[bench]
    fn bench_search_empty_letter(b: &mut Bencher) {
        b.iter(|| SHORTCUTS.search_range(b""));
    }

    #[test]
    fn search_not_found_letter() {
        let range = SHORTCUTS.search_range(b"zz");
        assert_eq!(range, None)
    }

    #[test]
    fn search_one_letter() {
        let range = SHORTCUTS.search_range(b"a");
        assert_eq!(range, Some(0..25417))
    }

    #[bench]
    fn bench_search_one_letter(b: &mut Bencher) {
        b.iter(|| SHORTCUTS.search_range(b"a"));
    }

    #[test]
    fn search_two_letter() {
        let range = SHORTCUTS.search_range(b"aa");
        assert_eq!(range, Some(1..29))
    }

    #[bench]
    fn bench_search_two_letter(b: &mut Bencher) {
        b.iter(|| SHORTCUTS.search_range(b"aa"));
    }

    #[test]
    fn search_three_letter() {
        let range = SHORTCUTS.search_range(b"aaa");
        assert_eq!(range, Some(1..29))
    }
}
