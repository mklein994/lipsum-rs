#[macro_use]
extern crate clap;
extern crate lipsum;
extern crate rand;

use rand::distributions::{Distribution, Uniform};

/// Define the command line arguments.
pub mod app;

/// Print a title.
pub fn title() {
    println!("{}", lipsum::lipsum_title());
}

/// Print `count` number of words.
pub fn words(count: usize) {
    println!("{}", lipsum::lipsum(count));
}

/// Print `count` number of paragraphs with 100 to 200 words each.
pub fn paragraphs(count: usize) {
    let mut rng = rand::thread_rng();
    let between = Uniform::from(100..200);

    for i in 0..count {
        let word_count = between.sample(&mut rng);
        println!("{}", lipsum::lipsum(word_count));

        // don't print a trailing newline if this is the last paragraph.
        if i < count - 1 {
            println!();
        }
    }
}
