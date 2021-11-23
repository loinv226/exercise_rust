use std::time::{Instant};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::rc::Rc;

mod algorithm;

use algorithm::*;

fn main() {
    // let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    // let strs = vec!["a".repeat(1000); 1000];

    let mut mems = vec![
        Algorithm::init(String::from("Loi"), || {linear::longest_common_prefix(vec!["a".repeat(1000); 1000])}, "a".repeat(1000)),
        Algorithm::init(String::from("Thanh"), || {fibonacci::longest_common_prefix(vec!["a".repeat(1000); 1000])}, "a".repeat(1000))
    ];
    // white let item = mems.g
    // {
    //     while let Some(item) = mems.iter().next() {
    //         item.run();
    //     }
    // }
    for m in mems.iter_mut() {
        m.run();
    }
    println!("Members: {:?}", mems);
}