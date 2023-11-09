use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    pub is_word: bool,
    pub children: HashMap<u8, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            is_word: false,
            children: HashMap::new(),
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Trie::new()
    }
}

impl Trie {
    pub fn append(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }
        self.append_bytes(value.as_bytes());
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        if let Some(letter) = bytes.first() {
            let node = self.children.entry(*letter).or_default();
            node.append_bytes(&bytes[1..]);
        } else {
            self.is_word = true;
        }
    }
}

impl Trie {
    pub fn follow(&self, bytes: &[u8]) -> Option<&Trie> {
        if let Some(byte) = bytes.first() {
            if let Some(node) = self.children.get(byte) {
                node.follow(&bytes[1..])
            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    pub fn dfs(&self, words: &mut Vec<String>, path: &[u8]) {
        if self.is_word {
            words.push(String::from_utf8_lossy(path).to_string());
        }
        for (letter, child) in &self.children {
            let mut new_path = path.to_vec();
            new_path.push(*letter);
            child.dfs(words, &new_path);
        }
    }
}

impl Trie {
    pub fn search(&self, value: &str) -> bool {
        self.search_bytes(value.as_bytes())
    }

    pub fn search_bytes(&self, bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return self.is_word;
        }
        if let Some(letter) = bytes.first() {
            if letter == &b'.' {
                return self.children.values().any(|c| c.search_bytes(&bytes[1..]));
            }
            if let Some(child) = self.children.get(letter) {
                return child.search_bytes(&bytes[1..]);
            }
        }
        false
    }
}

impl Trie {
    pub fn starts_with(&self, value: &str) -> Vec<String> {
        let bytes = value.as_bytes();
        let mut words = vec![];
        if let Some(root) = self.follow(bytes) {
            root.dfs(&mut words, bytes)
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let mut tr = Trie::default();
        tr.append("ba");
        tr.append("bad");
        tr.append("bac");
        tr.append("dad");
        tr.append("mad");

        assert!(!tr.search("b"));
        assert!(tr.search("ba"));
        assert!(tr.search("bad"));
        assert!(tr.search("mad"));
        assert!(!tr.search("bab"));
        assert!(!tr.search("pad"));

        tr.append("bab");
        assert!(tr.search("bab"));
    }

    #[test]
    fn case_with_dot() {
        let mut tr = Trie::default();
        tr.append("ba");
        tr.append("bad");

        assert!(!tr.search(".b"));
        assert!(!tr.search(".b."));
        assert!(!tr.search("b"));
        assert!(tr.search("bad"));
        assert!(tr.search("b.d"));
        assert!(tr.search(".a."));
        assert!(tr.search("b.."));
        assert!(tr.search("..d"));
    }

    #[test]
    fn prefix_search() {
        let mut tr = Trie::default();
        tr.append("ba");
        tr.append("bad");
        tr.append("bcd");

        assert_eq!(
            {
                let mut res = tr.starts_with("b");
                res.sort();
                res
            },
            vec!["ba", "bad", "bcd"]
        );
        assert_eq!(
            {
                let mut res = tr.starts_with("ba");
                res.sort();
                res
            },
            vec!["ba", "bad"]
        );
        assert_eq!(tr.starts_with("c"), Vec::<String>::new());
        assert_eq!(tr.starts_with("abc"), Vec::<String>::new());
    }
}
