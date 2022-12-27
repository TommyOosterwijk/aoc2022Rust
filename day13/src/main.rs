use std::slice::SliceIndex;

fn main() {
    println!("Day 13!");

    let instructions = include_str!("../day13.txt");
    let mut index_counter = 1;
    let mut result_A = 0;
    let look_up: Vec<&str> = vec!["0","1", "2","3","4","5","6","7","8","9","10"];

    let i: Vec<&str> = instructions.lines().into_iter().filter(|s|!s.is_empty()).collect();

    i.chunks(2).for_each(|s|{
        let s1 = s[0];
        let s2 = s[1];
        if is_valid_order(s1, s2, &look_up) {
            result_A+= index_counter;
        }

        index_counter+=1;        
    });

    println!("A: {}", result_A);

    let mut b_counter2 = 1;
    let mut b_counter6 = 2;

    for str in i.iter() {
        if is_valid_order(str, "[[2]]", &look_up) {
            b_counter2+=1;
        }
        if is_valid_order(str, "[[6]]", &look_up) {
            b_counter6+=1;
        }
    }
    println!("B: {}", b_counter2*b_counter6);

}

fn is_valid_order(s1: &str, s2: &str, look_up: &Vec<&str>) -> bool {
    let (mut char1,mut str1) = s1.split_at(1);
    if str1.starts_with("0") && char1 == "1" {
        (_, str1) = str1.split_at(1);
        char1 = "10";
    }

    let (mut char2,mut str2) = s2.split_at(1);
    if str2.starts_with("0") && char2 == "1" {
        (_, str2) = str2.split_at(1);
        char2 = "10";
    }

    if char1 == char2 {
        //println!("go next!");
        return is_valid_order(str1, str2, look_up);
    }

    if look_up.contains(&char1) && look_up.contains(&char2) {
        let i1: i32 = char1.parse().unwrap();
        let i2: i32 = char2.parse().unwrap();
        if i1 < i2 {
            return true;
        }
        return false;
    }

    if char1 == "]" {
        return true;
    }

    if char2 == "]" {
        return false;
    }

    if char1 == "[" {            
        let mut new_string = "[".to_owned() + &s2;
        if char2 == "10" {
            new_string.insert(3, ']');
        } else {
            new_string.insert(2, ']');
        }
        return is_valid_order(s1, &new_string, look_up);
    }

    if char2 == "[" {
        let mut new_string = "[".to_owned() + &s1;
            if char1 == "10" {
                new_string.insert(3, ']');
            } else {
                new_string.insert(2, ']');
            }
            return is_valid_order(&new_string, s2, look_up);
        }
        return false;
    }