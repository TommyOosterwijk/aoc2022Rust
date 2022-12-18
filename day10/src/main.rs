fn main() {
    println!("Day10");
    let instructions = include_str!("../day10.txt").split("\n");

    let mut x:i32 = 1;
    let mut value_holder = 0;
    let mut cycle_number = 1;
    let look_up: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut answer_a: i32 = 0;
    let mut string_value: Vec<char> = Vec::new();
    
    instructions.for_each(|s| {
    
        if cycle_number-1 >= x-1 && cycle_number-1 <= x+1 {
            string_value.push('#')
        } else {
            string_value.push('.')
        }
        if look_up.contains(&cycle_number) {
            answer_a += x * cycle_number;
        }

        if cycle_number == 40 {
            cycle_number = 0;
        }

        if !s.contains("noop") {
            let split = s.split_whitespace().collect::<Vec<&str>>();
            value_holder = split.get(1).unwrap().parse::<i32>().unwrap();
            cycle_number+=1;
            if cycle_number-1 >= x-1 && cycle_number-1 <= x+1 {
                string_value.push('#')
            } else {
                string_value.push('.')
            }
            if look_up.contains(&cycle_number) {
                answer_a += x * cycle_number;
            }
            if cycle_number == 40 {
                cycle_number = 0;
            }
        }

        cycle_number+=1;
        x += value_holder;
        value_holder = 0;
    });

println!("A: {}", answer_a);
println!("B:");
string_value.chunks(40).for_each(|crt_line| println!("{:?}", crt_line.iter().collect::<String>()));


}
