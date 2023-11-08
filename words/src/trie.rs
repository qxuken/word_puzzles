use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Trie {
    is_word: bool,
    children: HashMap<u8, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            is_word: false,
            children: HashMap::with_capacity(26),
        }
    }

    pub fn append(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }
        self.append_bytes(VecDeque::from(value.as_bytes().to_owned()));
    }

    fn append_bytes(&mut self, mut bytes: VecDeque<u8>) {
        if let Some(letter) = bytes.pop_front() {
            let node = if let Some(child) = self.children.get_mut(&letter) {
                child
            } else {
                self.children.insert(letter, Trie::new());
                self.children.get_mut(&letter).unwrap()
            };
            node.append_bytes(bytes);
        } else {
            self.is_word = true;
        }
    }

    pub fn search(&self, value: &str) -> bool {
        let mut search_bytes = VecDeque::from(value.as_bytes().to_owned());

        if search_bytes.is_empty() {
            return false;
        }

        let mut nodes_vec = vec![self];
        while let Some(letter) = search_bytes.pop_front() {
            let mut next_nodes = vec![];
            if letter == b'.' {
                while let Some(node) = nodes_vec.pop() {
                    let children = node.children.values();
                    if search_bytes.is_empty() && !node.children.is_empty() {
                        for child in children {
                            if child.is_word {
                                return child.is_word;
                            }
                        }
                        continue;
                    }
                    next_nodes.append(&mut children.collect());
                }
            } else {
                while let Some(node) = nodes_vec.pop() {
                    if let Some(child) = node.children.get(&letter) {
                        if search_bytes.is_empty() {
                            if child.is_word {
                                return true;
                            }
                            continue;
                        }
                        next_nodes.push(child);
                    }
                }
            }
            nodes_vec = next_nodes;
        }
        false
    }
}

impl Default for Trie {
    fn default() -> Self {
        Trie::new()
    }
}

#[derive(Debug)]
pub struct WordDictionary {
    words: Trie,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            words: Trie::default(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.append(word)
    }

    pub fn search(&self, word: &str) -> bool {
        self.words.search(word)
    }
}

impl Default for WordDictionary {
    fn default() -> Self {
        WordDictionary::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let mut wd = WordDictionary::new();
        wd.add_word("ba");
        wd.add_word("bad");
        wd.add_word("bac");
        wd.add_word("dad");
        wd.add_word("mad");

        println!("{:#?}", wd);

        assert!(!wd.search("pad"));
        assert!(wd.search("bad"));
        assert!(wd.search(".ad"));
        assert!(wd.search("b.."));
    }

    #[test]
    fn failed_case_1() {
        let mut wd = WordDictionary::new();
        wd.add_word("a");
        wd.add_word("a");

        println!("{:#?}", wd);

        assert!(wd.search("."));
        assert!(wd.search("a"));
        assert!(!wd.search("aa"));
        assert!(wd.search("a"));
        assert!(!wd.search(".a"));
        assert!(!wd.search("a."));
        assert!(!wd.search(".a."));
    }

    #[test]
    fn failed_case_2() {
        // ["WordDictionary","addWord","addWord","addWord","addWord","search","search","addWord","search","search","search","search","search","search"]
        // [[],              ["at"],   ["and"],  ["an"],   ["add"],  ["a"],   [".at"], ["bat"],  [".at"], ["an."], ["a.d."],["b."],  ["a.d"], ["."]]
        // [null,            null,     null,     null,     null,     false,   false,   null,     true,    true,    false,   false,   true,    false]
        let mut wd = WordDictionary::new();
        wd.add_word("at");
        wd.add_word("and");
        wd.add_word("an");
        wd.add_word("add");

        println!("{:#?}", wd);

        assert!(!wd.search("a"));
        assert!(!wd.search(".at"));

        wd.add_word("bat");

        assert!(wd.search(".at"));
        assert!(wd.search("an."));
        assert!(!wd.search("a.d."));
        assert!(!wd.search("b."));
        assert!(wd.search("a.d"));
        assert!(!wd.search("."));
    }
}
