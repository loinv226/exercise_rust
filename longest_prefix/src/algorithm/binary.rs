
pub fn longest_common_prefix(strs: Vec<String>) -> String {

    if let Some(first) = strs.get(0) {
        let mut l = 0
        let mut r = first.len();
        
        
        while l < r {
            let mid = l + (r -l) / 2;
            // check is prefix
            if strs.iter().any(|x| &x[..mid] != &first[..mid]) {
                r -= mid;
            } else {
                l += mid;
            }
        }
        if r == 0 {
            return String::from("");
        }
        return &first[..r]
    }
    return String::from("");
}