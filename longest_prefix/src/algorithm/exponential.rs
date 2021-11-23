
pub fn longest_common_prefix(strs: Vec<String>) -> String {

    if let Some(first) = strs.get(0) {
        let len = first.len();
        // println!("len: {:?}", len);
        if len == 0 {
            return String::from("");
        }
        let mut i = 1;
        
        while i <= len {
            if strs.iter().any(|x| &x[..i] != &first[..i]) {
                break;
            }
            if i == len {
                return first.clone();
            }
            i *= 8;
        }
        // println!("i: {:?}", i);
        
        
        let end = if i > len {len} else {i};
        let prev = end/8;
        // println!("prev: {:?} - end: {:?}", prev, end);
        return binary_search(strs, prev, end)
    }
    return String::from("");
}

fn binary_search(strs: Vec<String>, start: usize, end: usize) -> String {

    if let Some(first) = strs.get(0) {
        let mut l = start;
        let mut r = end;
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