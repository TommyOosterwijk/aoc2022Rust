fn main() {
    println!("Day06");

    let input = include_str!("../day6.txt").split("\n");
    let result: i32 = input.clone().map(| s | get_pos(s, 4, 4)).sum();

    println!("A: {}", result);

    let result_b: i32 = input.clone().map(| s | get_pos(s, 14, 14)).sum();

    println!("B: {}", result_b);
}

fn get_pos(s : &str, step: i32, step_value: i32) -> i32 {
    let ss: String = s.clone().chars().take(step.try_into().unwrap()).collect();
    let result = unique_chars(ss.as_str());
    if result {
        return step_value;
    } 
    let t: String = s.chars().skip(1).collect();
        return get_pos(&t, step, step_value + 1);
}

fn unique_chars(s: &str) -> bool {
    let mut v: Vec<char> = s.chars().collect();
    v.sort();
    let mut y = v.clone();
    y.dedup();
    v.len() == y.len()
}