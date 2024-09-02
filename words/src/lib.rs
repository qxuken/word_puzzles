#![feature(test, anonymous_lifetime_in_impl_trait)]
extern crate test;

pub use games::*;
pub use words_dict::WordsDict;
pub use words_shortcuts::*;
pub mod games;
pub mod words_dict;
pub mod words_shortcuts;
