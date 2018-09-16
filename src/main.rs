extern crate comrak;
extern crate glob;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let src_root = "/home/yk/monolog";

    for entry in glob::glob(&format!("{}/**/*.md", src_root)).expect("File list up failed") {
        let entry = entry.unwrap();

        println!("{}", entry.display());

        let mut f = File::open(entry).expect("file open failed");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("file read failed");
        println!("{}", markdown_to_html(&contents, &ComrakOptions::default()));
    }
}
