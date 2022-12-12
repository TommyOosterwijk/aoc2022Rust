fn main() {
    println!("Day5!");

    let input = include_str!("../day5.txt").split("\n");
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..40 {
        stacks.push(Vec::new());
    }

    let result: i32 = input.clone().take_while(|s| !s.contains('1')).map(|s| {

        let mut char_counter = 0;
        let mut index = 0;

        s.chars().for_each(|char| {
            if char == '[' {
                index = char_counter / 4;
            } else if char >= 'A' && char <= 'Z' {
                let s = stacks.get_mut(index).unwrap();
                s.insert(0, char);
            }
            char_counter+= 1;
        });  
        
        return 1;
    }).sum();

    let mut stacks_b = stacks.clone();

    input.clone().skip((result + 2).try_into().unwrap()).for_each(|s| {
        let splits = s.split_whitespace().collect::<Vec<&str>>();
        let to: usize = splits.get(5).unwrap().parse::<usize>().unwrap();
        let from: usize = splits.get(3).unwrap().parse::<usize>().unwrap();
        let count: i32 = splits.get(1).unwrap().parse::<i32>().unwrap();
        let mut temp: Vec<char> = Vec::new();

        for _ in 0..count {
            let s = stacks.get_mut(from -1).unwrap().pop().unwrap();
            stacks.get_mut(to-1).unwrap().push(s);
            let b = stacks_b.get_mut(from -1).unwrap().pop().unwrap();
            temp.push(b);
        }

        for _ in 0..temp.len() {
            stacks_b.get_mut(to-1).unwrap().push(temp.pop().unwrap());
        }
    });
    let result_a = get_row(stacks);
    println!("A: {}", result_a);
    let result_b = get_row(stacks_b);
    println!("B: {}", result_b); 
}

fn get_row(sss: Vec<Vec<char>>) -> String {
    let result: String = sss.into_iter().filter(|s| s.len() > 0).map(| s| {   
        let ss = s.get(s.len()-1).unwrap();
        return *ss;
    }).collect();
    return result;
}
