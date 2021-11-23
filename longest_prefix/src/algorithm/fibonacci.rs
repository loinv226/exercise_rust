use std::cmp;

fn common_prefix_of_two(left: &String, right: &String) -> String {
    let min_length = cmp::min(left.len(), right.len());
    for i in 0..min_length {
        // if left.chars().nth(i).unwrap() != right.chars().nth(i).unwrap() {
        //     return left[0..i].to_string();
        // }
        
        if &left[i..i+1] != &right[i..i+1] {
            return left[0..i].to_string();
        }
        
        
    }
    left[0..min_length].to_string()
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {

    if strs.len() == 1 {
        return strs[0].clone();
    }
     else if strs.len() == 2 {
        return common_prefix_of_two(&strs[0], &strs[1]);
    } else {
        let middle = strs.len() / 2;
        let left: String = longest_common_prefix(strs[0..middle+1].to_vec());
        let right: String = longest_common_prefix(strs[middle+1..].to_vec());
        return common_prefix_of_two(&left, &right);
    }
}