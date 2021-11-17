
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result: Vec<char> = vec![];
    let mut last_valid_index: i8 = 0;
   
    'rootLoop: for (i, elem) in strs.iter().enumerate() {
        for (j, value) in elem.chars().enumerate() {
            let j_index = j as i8;
            if i == 0 {
                // first work alway valid
                result.push(value);
                last_valid_index = j_index;
                continue
            }
            
            if let Some(exist) = result.get(j) {
                if *exist != value {
                    // not same prefix
                    last_valid_index = j_index - 1;
                    
                    if last_valid_index <= 0 {
                        break 'rootLoop;
                    }
                }
            }

            if j_index >= last_valid_index {
                break;
            }
        }
    }
    let mut output = String::from("");

    if last_valid_index < 0 {
        return output;
    }
    
    for (index, elem) in result.iter().enumerate() {
        if (index as i8) > last_valid_index {
            break;
        }
        output.push(*elem);
    }
    return output;
}

fn main() {
    let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let result = longest_common_prefix(strs);
    println!("{:?}", result)
}