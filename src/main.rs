#[macro_use]
extern crate clap;

mod app;

fn main() {
    let app = app::build_cli();
    let m = app.get_matches();
    println!("Hello, world!");
    println!("{:?}", m);
}
