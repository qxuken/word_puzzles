#![feature(test, anonymous_lifetime_in_impl_trait)]
extern crate test;

pub use games::*;
pub use words::*;
pub use words_file::WordsDict;
pub mod games;
pub mod words;
pub mod words_file;
