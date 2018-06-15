#[macro_use]
extern crate clap;
extern crate lipsum;

pub mod app;

pub use lipsum::lipsum;

/// Generate 'count' number words.
pub fn generate_words(count: usize) -> String {
    lipsum(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_10_words() {
        let count = 0;
        let words = generate_words(count);
        assert_eq!(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.",
            words
        );
    }
}
