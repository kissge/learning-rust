extern crate comrak;
use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    println!("{}", markdown_to_html("Hello, **世界**!", &ComrakOptions::default()));
}
