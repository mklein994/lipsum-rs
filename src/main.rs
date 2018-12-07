fn main() {
    if let Err(e) = lipsum_rs::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
