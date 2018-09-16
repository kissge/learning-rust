extern crate comrak;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;

fn main() {
    let src_dir = "/home/yk/monolog/";
    recurse(Path::new(src_dir))
        .expect("recurse failed");
}

fn recurse(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            recurse(&path)?;
        } else {
            println!("{}", path.to_str().unwrap());
            let mut f = File::open(path).expect("file not found");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("file read failed");

            println!("{}", markdown_to_html(&contents, &ComrakOptions::default()));
        }
    }
    Ok(())
}
