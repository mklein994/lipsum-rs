extern crate lipsum_rs;

use lipsum_rs::app;

fn main() {
    let m = app::build_cli().get_matches();

    println!("Hello, world!");
    println!("{:?}", m);
}
