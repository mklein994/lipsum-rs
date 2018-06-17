use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .arg(
            Arg::with_name("title")
                .help("Generate a title")
                .long("title")
                .conflicts_with("words")
                .short("t"),
        )
        .arg(
            Arg::with_name("words")
                .help("Number of words to generate")
                .index(1)
                .default_value("10")
                .number_of_values(1),
        )
}
