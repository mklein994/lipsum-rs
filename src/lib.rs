#[macro_use]
extern crate clap;
extern crate lipsum;

pub mod app;

pub use lipsum::lipsum;

/// Generate 'count' number words.
pub fn generate_words(count: usize) {
    println!("{}", lipsum(count));
}
