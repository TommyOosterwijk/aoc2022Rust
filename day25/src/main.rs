fn main() {
    println!("Day25");

    let lines:i128 = include_str!("../day25.txt").lines().into_iter().map(|s|str_to_int(s)).sum();

    println!("A: {}", lines);
    println!("B: {}", int_to_str(lines));
}

fn int_to_str(value: i128) -> String {
    let mut target = "1".to_string();
    while str_to_int(&target) <=  value {
        target = format!("{}0",target);
    }

    let mut final_string: Vec<String> = vec!["=".to_string(); target.len()-1 as usize];
    let look_up: Vec<String> = Vec::from(["2".to_string(), "1".to_string(), "0".to_string(), "-".to_string(), "=".to_string(),]);
    for i in 0..final_string.len() {
        for t in 0..5 {
            
            let s: String = look_up[t].clone();
            final_string[i] = s;    
                       
            let temp_s: String = final_string.clone().into_iter().collect();
            let t = str_to_int(&temp_s);

            if t == value {
                return temp_s;
            } else if t < value && t > 0 {
                break;
            } 
        }
    }

    return "".to_string();
}

fn str_to_int(s: &str) -> i128 {
    let mut result: i128= 0;
    let mut pos = 1;
    for ch in s.chars().rev() {        
        if ch == '1' {
            result+= pos;
        } else if ch == '2' {
            result+= 2 * (pos);
        } else if ch == '0' {}
        else if ch == '-' {
            result+= -1 * (pos);
        }
        else if ch == '=' {
            result+= -2 * (pos);
        }
        pos*=5;
    }
    return result;
}
