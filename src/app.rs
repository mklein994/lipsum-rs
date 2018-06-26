use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .arg(
            Arg::with_name("title")
                .help("Generate a title")
                .long("title")
                .short("t"),
        )
        .arg(
            Arg::with_name("words")
                .default_value("150")
                .help("Number of words to generate")
                .long("words")
                .number_of_values(1)
                .short("w"),
        )
        .arg(
            Arg::with_name("paragraphs")
                .conflicts_with("words")
                .help("Number of paragraphs to generate")
                .long("paragraphs")
                .short("p")
                .min_values(0)
                .max_values(1),
        )
}
