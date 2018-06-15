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
            Arg::with_name("paragraph")
                .help("Number of paragraphs to generate")
                .long("paragraph")
                .number_of_values(1)
                .short("p"),
        )
}
