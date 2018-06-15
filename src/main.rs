extern crate lipsum_rs;
#[macro_use]
extern crate clap;

use lipsum_rs::app;
use lipsum_rs::generate_words;

fn main() {
    let m = app::build_cli().get_matches();

    if m.is_present("words") {
        let count = value_t!(m.value_of("words"), usize).unwrap();
        println!("{}", generate_words(count));
    }
}
