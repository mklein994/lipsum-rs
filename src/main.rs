fn main() {
    let matches = lipsum_rs::app::build_cli().get_matches();

    if let Err(e) = lipsum_rs::run(matches) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
