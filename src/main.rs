extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let barcode = &args[1];
    println!("Searching for {}", barcode);
    get_product_name(barcode);
}

fn get_product_name(barcode: &str) {
    let resp = reqwest::blocking::get(format!("https://barcode.monster/code/{}", barcode)).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    let product_name = document
        .find(Class("card-body").descendant(Name("h2")))
        .next()
        .unwrap()
        .text();
        println!("{}\n",product_name);
}
