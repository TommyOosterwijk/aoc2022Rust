fn main() {
    println!("Day3!");

    let input = include_str!("../day3.txt").split("\n");

    let result: i32 = input.clone().map(| s | get_value(s)).map(|i| convert_value(i)).sum();
    println!(" A: {}", result);


    let lines = input.clone().collect::<Vec<_>>();
    let mut vec=Vec::new();

    for group in lines.chunks(3) {
        let result = get_values(group.get(0).unwrap(), group.get(1).unwrap(), group.get(2).unwrap());
        vec.push(result);
    }
    let result_b : i32 = vec.into_iter().map(|i| convert_value(i)).sum();
    println!(" B: {}", result_b);

}

fn convert_value(s: String) -> i32{
    let mut value = 0;
    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            value = (c as u32 - 96).try_into().unwrap();
        } else {
            value = (c as u32 - 38).try_into().unwrap();
        }
    }
    return value;
}

fn get_values(s1 : &str, s2 : &str, s3 : &str) -> String {

    return s1.chars().filter(|s| {
            return s2.contains(s.to_owned());
    }).filter(|s| {
        return s3.contains(s.to_owned());
    }).take(1).collect();
}


fn get_value(s : &str) -> String {
    let (split1, split2) = s.split_at(s.len()/2);

    return split1.chars().filter(|s| {
            return split2.contains(s.to_owned());
    }).take(1).collect();
}

