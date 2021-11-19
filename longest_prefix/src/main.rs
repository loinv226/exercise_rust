use std::time::Instant;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::from("");

    if let Some(first) = strs.get(0) {
        // let mut i = 0;
        for c in first.chars() {
            // println!("{:?}", i);
            // i += 1;
            prefix.push(c);
            // loop other word
            if strs.iter().any(|x| x.find(&prefix) != Some(0)) {
                prefix.pop();
                return prefix;
            }
        }
    }
    return prefix;
}

fn main() {
    let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    // let strs = vec!["a".repeat(1000); 1000];
    let now = Instant::now();
    let result = longest_common_prefix(strs);
    let dur = now.elapsed();
    println!("dur: {:.2?}", dur);
    println!("{:?}", result)
}