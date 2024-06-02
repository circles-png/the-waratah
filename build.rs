use std::{
    env::var,
    fs::{read_dir, read_to_string, File},
    io::Write,
};

fn main() {
    println!("cargo:rerun-if-changed=src/articles");
    let articles: Vec<_> = read_dir("src/articles")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let article = read_to_string(entry.path()).unwrap();
            format!(
                "{} {} {}",
                article.len() + 1 + entry.file_name().len(),
                entry.file_name().to_string_lossy(),
                article
            )
        })
        .collect();
    let articles = articles.join("\n");
    File::create(var("OUT_DIR").unwrap() + "/articles")
        .unwrap()
        .write_all(articles.as_bytes())
        .unwrap();
}
