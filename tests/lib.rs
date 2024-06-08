extern crate readability;
extern crate url;

use std::{fs::File, io::Read};
use url::Url;

fn read_file_to_string(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content).unwrap();
    content
}

#[test]
fn test_extract_title() {
    let content = read_file_to_string("./data/title.html");

    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(content.as_str(), &url).unwrap();
    assert_eq!(product.title, "This is title");
}

#[test]
fn test_fix_rel_links() {
    let content = read_file_to_string("./data/rel.html");

    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(content.as_str(), &url).unwrap();
    assert_eq!(product.content, "<!DOCTYPE html><html><head><title>This is title</title></head><body><p><a href=\"https://example.com/poop\"> poop </a></p></body></html>");
}

#[test]
fn test_fix_img_links() {
    let content = read_file_to_string("./data/img.html");

    let url = Url::parse("https://example.com").unwrap();
    let product = readability::extractor::extract(content.as_str(), &url).unwrap();
    assert_eq!(product.content, "<!DOCTYPE html><html><head><title>This is title</title></head><body><p><img src=\"https://example.com/poop.png\"></p></body></html>");
}
