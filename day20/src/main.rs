#[derive(Debug, Clone, Copy)]
struct number {
    value: i128,
    original_val:i128,
    index: i32
}
fn main() {
    
    println!("Day20");
    let mut index = 0;
    let mut lines: Vec<number> = include_str!("../day20.txt").lines().map(|s| {
        index+=1;
        return number { value: (s.parse().unwrap()), original_val: (s.parse().unwrap()), index: (index-1) }
    }).collect();
    index = 0;
    let mut lines_b: Vec<number> = include_str!("../day20.txt").lines().map(|s| {
        let mut num: i128 = s.parse().unwrap();
        num*= 811589153;        
        let mut val_to_use = num.rem_euclid(lines.len() as i128);
        if num < 0 {
            val_to_use = val_to_use - (val_to_use *2);
        }
        index+=1;
        return number { value: (num), original_val: (num), index: (index-1) }
    }).collect();

    println!("A: {}", hussle_the_map(&mut lines));

    
    for _ in 0..10 {
        println!("B: {}", hussle_the_map(&mut lines_b));
    }
}

fn hussle_the_map(lines: &mut Vec<number>) -> i128{
    let lines_len = lines.len() as i128;
    for index in 0..lines.len() {
        let mut number_target = lines.get(index).unwrap();
        let mut target_to_remove = 0;
        for i in 0..lines.len() {
            if lines.get(i).unwrap().index == index.try_into().unwrap() {
                target_to_remove = i;
                number_target = lines.get(i).unwrap();
                break;
            }
        }
        let mut val = number_target.value.clone();
        let mut target_v = number_target.value.clone();
        let or_val = number_target.original_val;
        let save_index = number_target.index;

        target_v = target_v.rem_euclid((lines_len-1) as i128) ;
        lines.remove(target_to_remove);
        let mut new_index: i128 = ((target_to_remove as i128 + target_v).rem_euclid((lines_len-1) as i128));

        if new_index < 0 {
            new_index = lines_len - new_index.abs() -1;
        }
        lines.insert(new_index as usize, number { value: (val), original_val: (or_val), index: (save_index)});
    }
    let mut result = 0;
    let mut zero_index = 0;
    for i in 0..lines.len() {
        if lines[i].value == 0 {
            zero_index = i;
            break;
        }
    }
    result+= lines.get((1000+zero_index) % (lines.len())).unwrap().original_val;
    result+= lines.get((2000+zero_index) % (lines.len())).unwrap().original_val;
    result+= lines.get((3000+zero_index) % (lines.len())).unwrap().original_val;
    return result;
}