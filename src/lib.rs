#[macro_use]
extern crate clap;
extern crate lipsum;
extern crate rand;

use rand::distributions::{Distribution, Uniform};

pub mod app;

pub fn get_paragraphs(count: usize) -> String {
    let mut p: String = String::new();
    let mut rng = rand::thread_rng();
    let between = Uniform::from(100..200);

    for _ in 0..count {
        let word_count = between.sample(&mut rng);
        p.push_str(&format!("{}\n\n", lipsum::lipsum(word_count)));
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_10_paragraphs() {
        const N: usize = 10;
        let actual = get_paragraphs(N);
        let lines = actual.lines();
        // Exclude the empty lines
        assert_eq!(N, lines.count() / 2);
    }
}
