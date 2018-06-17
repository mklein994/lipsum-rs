#[macro_use]
extern crate clap;
extern crate lipsum;

pub mod app;

pub fn get_paragraphs(count: usize) -> String {
    let mut p: String = String::new();
    for _ in 0..count {
        p.push_str(&format!("{}\n", lipsum::lipsum(1)));
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_10_paragraphs() {
        let actual = get_paragraphs(10);
        assert_eq!(10, actual.lines().count());
    }
}
