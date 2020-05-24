extern crate comrak;

use comrak::{markdown_to_html, ComrakOptions};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let output = &args[2];
    let path = Path::new(output);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let html = markdown_to_html(&contents, &ComrakOptions::default());
    println!("{}", html);

    match file.write_all(html.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
