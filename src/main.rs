extern crate comrak;

use comrak::{markdown_to_html, ComrakOptions};
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    println!("{}", markdown_to_html(&contents, &ComrakOptions::default()));
}
