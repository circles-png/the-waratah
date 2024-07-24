use std::{
    env::var,
    fs::{read_dir, read_to_string, File},
    io::Write,
};

use itertools::Itertools;
use proc_macro2::TokenTree;
use syn::{parse_file, Item, ItemFn, Signature, Stmt, StmtMacro};

fn main() {
    println!("cargo:rerun-if-changed=src/articles");
    println!("cargo:rerun-if-changed=src/crosswords");
    println!("cargo:rerun-if-changed=src/images/ads");
    let article_ids = collect_articles();
    collect_ads();
    let crosswords = collect_crosswords();
    generate_sitemap(&article_ids, crosswords);

    dbg!(var("OUT_DIR").unwrap());
}

fn generate_sitemap(article_ids: &[String], crosswords: usize) {
    let sitemap = parse_file(include_str!("src/components.rs"))
        .unwrap()
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Fn(ItemFn {
                sig: Signature { ident, .. },
                block,
                ..
            }) if (*ident == "App") => Some(
                block
                    .stmts
                    .iter()
                    .filter_map(|statement| match statement {
                        Stmt::Macro(StmtMacro { mac, .. }) => {
                            let tokens = mac.tokens.clone().into_iter().collect_vec();
                            Some(
                                tokens
                                    .split(|token| {
                                        matches!(
                                            token,
                                            TokenTree::Punct(punct)
                                                if punct.as_char() == '='
                                        )
                                    })
                                    .filter_map(|sub| (!sub.is_empty()).then_some(sub.to_vec()))
                                    .tuple_windows()
                                    .filter_map(|(a, b)| {
                                        let [second_last, last] = a.last_chunk().unwrap();
                                        matches!(
                                            (second_last, last),
                                            (
                                                TokenTree::Ident(second_last),
                                                TokenTree::Ident(last)
                                            )
                                            if *second_last == "Route" && *last == "path"
                                        )
                                        .then(|| b.first().unwrap().clone())
                                        .and_then(
                                            |token| {
                                                let TokenTree::Literal(literal) = token else {
                                                    return None;
                                                };
                                                Some(
                                                    literal
                                                        .to_string()
                                                        .strip_prefix('"')
                                                        .unwrap()
                                                        .strip_suffix('"')
                                                        .unwrap()
                                                        .to_string(),
                                                )
                                            },
                                        )
                                    })
                                    .collect_vec(),
                            )
                        }
                        _ => None,
                    })
                    .exactly_one()
                    .unwrap(),
            ),
            _ => None,
        })
        .exactly_one()
        .unwrap()
        .iter()
        .flat_map(|route| match route.as_str() {
            "/articles/:id" => article_ids
                .iter()
                .map(|id| format!("/articles/{id}"))
                .collect_vec(),
            "/crosswords/:id" => (0..crosswords)
                .map(|id| format!("/crosswords/{id}"))
                .collect_vec(),
            "/*" => Vec::new(),
            _ => {
                vec![route.clone()]
            }
        })
        .collect_vec()
        .join("\n");
    File::create(var("OUT_DIR").unwrap() + "/sitemap.txt")
        .unwrap()
        .write_all(sitemap.as_bytes())
        .unwrap();
}

fn collect_crosswords() -> usize {
    let crosswords = read_dir("src/crosswords")
        .unwrap()
        .map(|entry| {
            read_to_string(entry.unwrap().path())
                .unwrap()
                .trim()
                .to_string()
        })
        .collect_vec();
    File::create(var("OUT_DIR").unwrap() + "/crosswords")
        .unwrap()
        .write_all(
            &crosswords
                .join("\n\n")
                .bytes()
                .map(u8::reverse_bits)
                .collect_vec(),
        )
        .unwrap();
    crosswords.len()
}

fn collect_articles() -> Vec<String> {
    let topics = read_dir("src/articles").unwrap();
    let (articles, id): (Vec<_>, Vec<_>) = topics
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
                    let id = entry.file_name();
                    let id = id.to_string_lossy();
                    let data = format!("{} {} {} {}", topic.len(), topic, id, article);
                    (format!("{} {}", data.len(), data), id.to_string())
                })
                .collect_vec()
        })
        .unzip();
    File::create(var("OUT_DIR").unwrap() + "/articles")
        .unwrap()
        .write_all(articles.into_iter().collect_vec().join("\n").as_bytes())
        .unwrap();
    id
}

fn collect_ads() {
    let ads: Vec<_> = read_dir("src/images/horizontal-ads")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    File::create(var("OUT_DIR").unwrap() + "/ads")
        .unwrap()
        .write_all(ads.join("\n").as_bytes())
        .unwrap();
}
