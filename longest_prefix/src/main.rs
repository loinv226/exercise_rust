
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut output = String::from("");

    if let Some(first) = strs.get(0) {
        for (i, c) in first.chars().enumerate() {
            // loop other word
            for (j, other) in strs.iter().enumerate() {
                if j == 0 {
                    continue;
                }
                if other.len() <= i || c != other.chars().nth(i).unwrap() {
                    return output;
                }
            }
            output.push(c);
        }
    }
    return output;
}

fn main() {
    let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let result = longest_common_prefix(strs);
    println!("{:?}", result)
}