
pub fn longest_common_prefix(strs: Vec<String>) -> String {

    if let Some(first) = strs.get(0) {
        let mut l = 0;
        let mut r = first.len();
        if r == 0 {
            return String::from("");
        }
        
        while l <= r {
            let mid = (l + r) / 2;
            // println!("mid: {:?}", mid);
            // check is prefix
            if strs.iter().any(|x| &x[..mid] != &first[..mid]) {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return first[..(l + r) / 2].to_string();
    }
    return String::from("");
}