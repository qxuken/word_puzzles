// use std::{thread, time};
use words::TRIE;
use words::WORDS;

fn main() {
    println!("{:?}", TRIE.starts_with("abc"));
    println!(
        "{:?}",
        WORDS
            .iter()
            .filter(|word| word.starts_with(b"abc"))
            .map(|word| String::from_utf8_lossy(word).to_string())
            .collect::<Vec<String>>()
    );

    // let ten_millis = time::Duration::from_secs(60);
    // let now = time::Instant::now();

    // thread::sleep(ten_millis);

    // assert!(now.elapsed() >= ten_millis);
}
