
fn main() {
    let input = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let mut result: Vec<char> = vec![];
    let mut last_valid_index: i8 = 0;
   
    'rootLoop: for (i, elem) in input.iter().enumerate() {
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
    if last_valid_index < 0 {
        println!("NOT_FOUND");
        return;
    }
    let mut output = String::from("");

    for (index, elem) in result.iter().enumerate() {
        if (index as i8) > last_valid_index {
            break;
        }
        output.push(*elem);
    }
    println!("{:?}", output)
}


// Input: strs = ["flower","flow","flight"]
// Output: "fl"