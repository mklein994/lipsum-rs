use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .arg(
            Arg::with_name("title")
                .help("Generate a title")
                .long("title")
                .conflicts_with("paragraph")
                .short("t"),
        )
        .arg(
            Arg::with_name("words")
                .help("Number of words to generate")
                .long("words")
                .number_of_values(1)
                .short("w"),
        )
}
