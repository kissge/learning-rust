extern crate comrak;
extern crate glob;
use comrak::{markdown_to_html, ComrakOptions};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let src_root = "/mnt/c/files/monolog/src";
    let dst_root = "/mnt/c/files/monolog/dst";

    env::set_current_dir(Path::new(src_root)).expect(&format!("Cannot cd to {}", src_root));

    let mut entries = HashMap::new();

    for src_path in glob::glob("**/*.md").expect("File list up failed") {
        let src_path = src_path.unwrap();
        let src_path = src_path.to_str().unwrap();
        assert!(src_path.ends_with(".md"));
        let canonical_path = src_path.get(..src_path.len() - 3).unwrap();
        let dst_path = &format!("{}/{}.html", dst_root, canonical_path);

        fs::create_dir_all(Path::new(&dst_path).parent().unwrap())
            .expect("directory create failed");

        println!("{} -> {}", src_path, dst_path);

        let mut f = File::open(&src_path).expect("file open failed");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("file read failed");

        let mut f = File::create(&dst_path).expect("file create failed");
        f.write_all(markdown_to_html(&contents, &ComrakOptions::default()).as_bytes())
            .expect("file write failed");

        let entry_path = str::replace(&dst_path, dst_root, "");
        entries.insert(canonical_path.to_string(), entry_path);
    }

    let mut f = File::create(&format!("{}/index.html", dst_root)).expect("file create failed");
    f.write(b"<ol>\n").unwrap();
    for (key, entry) in &entries {
        f.write(&format!("<li><a href=\"{}\">{}</a></li>\n", entry, key).as_bytes())
            .unwrap();
    }
    f.write(b"</ol>\n").unwrap();
}
