extern crate comrak;
extern crate glob;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let src_root = "/home/yk/monolog";
    let dst_root = "/tmp/monolog";

    let mut entries = vec![];

    for src_path in glob::glob(&format!("{}/**/*.md", src_root)).expect("File list up failed") {
        let src_path = src_path.unwrap();
        let dst_path = str::replace(src_path.to_str().unwrap(), src_root, dst_root);
        let dst_path = str::replace(&dst_path, ".md", ".html");

        fs::create_dir_all(Path::new(&dst_path).parent().unwrap()).expect("directory create failed");

        println!("{} -> {}", src_path.display(), dst_path);

        let mut f = File::open(&src_path).expect("file open failed");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("file read failed");

        let mut f = File::create(&dst_path).expect("file create failed");
        f.write_all(markdown_to_html(&contents, &ComrakOptions::default()).as_bytes()).expect("file write failed");

        let entry_path = str::replace(&dst_path, dst_root, "");
        entries.push(entry_path);
    }

    let mut f = File::create(&format!("{}/index.html", dst_root)).expect("file create failed");
    f.write(b"<ol>").unwrap();
    for entry in entries {
        f.write(&format!("<li><a href=\"{}\">{}</a></li>", entry, entry).as_bytes()).unwrap();
    }
    f.write(b"</ol>").unwrap();
}
