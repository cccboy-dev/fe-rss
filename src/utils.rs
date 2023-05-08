use std::io::Read;

use brotli::Decompressor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum RssItemLink {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize)]
struct RssItem {
    title: String,
    rss: RssItemLink,
}

#[derive(Serialize, Deserialize)]
struct LinkChildItem {
    title: String,
    link: String,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct LinkItem {
    title: String,
    items: Vec<LinkChildItem>,
}

#[derive(Serialize, Deserialize)]
struct TagItem {
    tag: String,
    filename: String,
    keywords: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "updateTime")]
    update_time: String,
    rss: Vec<RssItem>,
    tags: Vec<TagItem>,
    links: Vec<LinkItem>,
}

pub fn read_remote_data() -> Result<Data, Box<dyn std::error::Error>> {
    let url = "https://fed.chanceyu.com/data.json.br";
    let response = reqwest::blocking::get(url)?.error_for_status()?;

    let mut decompressed = String::new();
    let mut decoder = Decompressor::new(response, 4096);
    decoder.read_to_string(&mut decompressed)?;

    let data: Data = serde_json::from_str(&decompressed)?;

    Ok(data)
}

fn demo() {
    let data = read_remote_data().unwrap();
}
