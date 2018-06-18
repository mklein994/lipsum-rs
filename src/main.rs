extern crate lipsum_rs;
#[macro_use]
extern crate clap;
extern crate lipsum;

use lipsum_rs::app;

fn main() {
    let m = app::build_cli().get_matches();

    if m.is_present("title") {
        println!("{}", lipsum::lipsum_title());
    } else if m.is_present("paragraphs") {
        let count = value_t!(m.value_of("paragraphs"), usize).unwrap();
        println!("{}", lipsum_rs::get_paragraphs(count));
    } else {
        let count = value_t!(m.value_of("words"), usize).unwrap();
        println!("{}", lipsum::lipsum(count));
    }
}
