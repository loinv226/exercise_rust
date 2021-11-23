
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::from("");

    if let Some(first) = strs.get(0) {
        // let mut i = 0;
        for c in first.chars() {
            // println!("{:?}", i);
            // i += 1;
            prefix.push(c);
            // check other word contain any item not valid
            if strs.iter().any(|x| &x[..prefix.len()] != &prefix[..]) {
                prefix.pop();
                return prefix;
            }
        }
    }
    return prefix;
}