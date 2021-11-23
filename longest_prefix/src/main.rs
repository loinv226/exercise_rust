// use serde::{Serialize, Deserialize};
// use serde_json::Result;
mod algorithm;

use algorithm::*;

fn main() {
    // let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    // let strs = vec!["a".repeat(1000); 1000];

    let repeat = &10000;
    let char_repeat = &1000;
    let mut mems = vec![
        Algorithm::init(String::from("linear"), || {linear::longest_common_prefix(vec!["a".repeat(*repeat); *repeat])}, "a".repeat(*repeat)),
        Algorithm::init(String::from("binary"), || {binary::longest_common_prefix(vec!["a".repeat(*char_repeat); *repeat])}, "a".repeat(*char_repeat)),
        Algorithm::init(String::from("exponential"), || {exponential::longest_common_prefix(vec!["a".repeat(*char_repeat); *repeat])}, "a".repeat(*char_repeat)),
        Algorithm::init(String::from("fibonacci"), || {fibonacci::longest_common_prefix(vec!["a".repeat(*repeat); *repeat])}, "a".repeat(*repeat)),
    ];
    // Algorithm::init(String::from("exponential"), || {exponential::longest_common_prefix(vec![String::from("aaaaaabb"), String::from("aaaaaacc"), String::from("aaaaaadd")])}, String::from("aaaaaa")),

    for m in mems.iter_mut() {
        m.run();
    }
    // println!("Members: {:?}", mems);
}