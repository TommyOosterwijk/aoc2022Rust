fn main() {
    println!("Day7!");
    let mut vec: Vec<i32> = Vec::new();
    let mut index = 0;
    let mut instructions = include_str!("../day7.txt").split("\n").skip(1).collect::<Vec<&str>>();
    do_instructions(&mut instructions, &mut index, &mut vec);
    
    vec.sort();
    let result_a: i32 = vec.clone().iter().filter(|v| **v < 100000).sum();
    println!("A: {}", result_a);

    let space_used = vec.get(vec.len()-1).unwrap();
    let diff = 30000000 - (70000000 - space_used);
    let result_b = vec.iter().filter(|v| **v >= diff).collect::<Vec<&i32>>();
    println!("B: {}", result_b.get(0).unwrap());
}

fn do_instructions(instrucions: &mut Vec<&str>, index: &mut usize, dir_size: &mut Vec<i32>) -> i32{
    let mut result = 0;
    let mut instruction = instrucions.get(*index).unwrap();
    let max_size = instrucions.len() -1;
    while !instruction.contains("$ cd ..") && *index <= max_size {
        *index+=1;

        if instruction.contains("$ cd") {
            result = do_instructions(instrucions, index, dir_size);
        } else if !instruction.contains("dir") && !instruction.contains("$ ls") {
            let vec = instruction.split_whitespace().collect::<Vec<&str>>();
            result += vec.get(0).unwrap().parse::<i32>().unwrap();
        }   
        if *index > max_size {
            break;
        }
        instruction = instrucions.get(*index).unwrap();
    }

    if *index < max_size {
        *index+=1;
    }
    dir_size.push(result);
    return result;
}