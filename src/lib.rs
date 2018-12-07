#[macro_use]
extern crate clap;

use rand::distributions::{Distribution, Uniform};

/// Define the command line arguments.
pub mod app;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let m = app::build_cli().get_matches();

    if m.is_present("title") {
        println!("{}", lipsum::lipsum_title());

        // If there are more orguments, or if `-w` was passed, add a newline after the title.
        if m.args.len() > 2 || m.occurrences_of("words") == 1 {
            println!();
        }
    }

    if m.is_present("paragraphs") {
        let count = value_t!(m.value_of("paragraphs"), usize).unwrap_or(3);
        paragraphs(count);
    }

    if m.occurrences_of("words") == 1 || m.args.len() == 1 {
        let count = value_t!(m.value_of("words"), usize)?;
        words(count);
    }
    Ok(())
}

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
