extern crate lipsum_rs;
#[macro_use]
extern crate clap;
extern crate lipsum;

use lipsum_rs::app;

fn run() -> Result<(), Box<std::error::Error>> {
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
        lipsum_rs::paragraphs(count);
    }

    if m.occurrences_of("words") == 1 || m.args.len() == 1 {
        let count = value_t!(m.value_of("words"), usize)?;
        lipsum_rs::words(count);
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
