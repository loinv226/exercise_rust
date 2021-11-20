use std::time::Instant;
use std::cmp;

mod config;
mod loi;
mod thanh;

use config::*;

fn main() {
    // let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let strs = vec!["a".repeat(1000); 1000];
    let now = Instant::now();
    
    match YOUR_CHOOSE {
        Choose::Loi => {
            let result = loi::longest_common_prefix(strs);
            println!("{:?}", result);
        }
        Choose::Thanh => {
            let result = thanh::longest_common_prefix(strs);
            println!("{:?}", result);
        }
    }
    
    let dur = now.elapsed();
    println!("dur: {:.2?}", dur);
    
}