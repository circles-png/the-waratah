use std::{
    env::var,
    fs::{read_dir, read_to_string, File},
    io::Write,
};

fn main() {
    println!("cargo:rerun-if-changed=src/articles");
    collect_articles();
    collect_ads();
}

fn collect_articles() {
    let topics = read_dir("src/articles").unwrap();
    let articles: Vec<_> = topics
        .flat_map(|topic_entry| {
            let topic_entry = &topic_entry.unwrap();
            read_dir(topic_entry.path())
                .unwrap()
                .map(|article_entry| {
                    let entry = article_entry.unwrap();
                    let article = read_to_string(entry.path()).unwrap();
                    let article = article.trim();
                    let topic = topic_entry.file_name();
                    let topic = topic.to_string_lossy();
                    let data = format!(
                        "{} {} {} {}",
                        topic.len(),
                        topic,
                        entry.file_name().to_string_lossy(),
                        article
                    );
                    format!("{} {}", data.len(), data)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let articles = articles.join("\n");
    File::create(var("OUT_DIR").unwrap() + "/articles")
        .unwrap()
        .write_all(articles.as_bytes())
        .unwrap();
}

fn collect_ads() {
    let ads: Vec<_> = read_dir("src/images/ads")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    let ads = ads.join("\n");
    File::create(var("OUT_DIR").unwrap() + "/ads")
        .unwrap()
        .write_all(ads.as_bytes())
        .unwrap();
}
