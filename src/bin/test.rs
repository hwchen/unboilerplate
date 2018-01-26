extern crate unboilerplate;

use std::fs::File;
use std::io::Read;
use unboilerplate::unboilerplate;

fn main() {
    let mut f = File::open("test-data/finalrun-input/103.html").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf);

    println!("{}", unboilerplate(&buf).unwrap());
}
