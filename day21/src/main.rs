use scanf::sscanf;

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    value: i128,
    left_val: i128,
    right_val: i128,
    list: Vec<String>,
    action: String
}

fn main() {
    println!("Day21 B!");
    let mut list: Vec<Monkey> = Vec::new();
    let mut monkeys: Vec<Monkey> = include_str!("../day21.txt").lines().map(|s|{
        let mut name = "".to_string();
    
        if s.len() > 10 {
            let mut left_part = "".to_string();
            let mut right_part = "".to_string();
            let mut action = "".to_string();

            sscanf!(&s,"{}: {} {} {}", name, left_part, action, right_part)
            .expect("error parsing instruction");

            let mut list:Vec<String> = Vec::new();
            list.push(left_part);
            list.push(right_part);

            return Monkey {name: name, value: 0, left_val: 0, right_val: 0, list: list, action: action}

        } 
        let mut value = 0;

        sscanf!(&s,"{}: {}", name, value)
            .expect("error parsing instruction");

            list.push(Monkey {name: name.clone(), value: value, left_val: 0, right_val: 0, list: Vec::new(), action: "".to_string()});
        return Monkey {name: name, value: value, left_val: 0, right_val: 0, list: Vec::new(), action: "".to_string()}
    }).collect();

    let mut target = 3587647562850;

    for index in target..target+5 {
        //for index in 0..100 {
        let mut temp_monkeys:Vec<Monkey> = Vec::new();

        for m in monkeys.iter() {
            let mut l: Vec<String> = Vec::new();
            for s in m.list.iter() {
                l.push(s.clone());
            }
            temp_monkeys.push(Monkey {name: m.name.clone(), value: m.value, left_val: m.left_val, right_val: m.right_val, list: l, action: m.action.clone()});
        }

        let mut temp_list:Vec<Monkey> = Vec::new();
        for m in list.iter() {
            let mut l: Vec<String> = Vec::new();
            let mut value = m.value;

            if m.name.contains("humn") {
                value = index;
            }
            for s in m.list.iter() {
                l.push(s.clone());
            }
            temp_list.push(Monkey {name: m.name.clone(), value: value, left_val: m.left_val, right_val: m.right_val, list: l, action: m.action.clone()});
        }
        resolve_monkeys(&mut temp_monkeys, &mut temp_list);

        let mut target_found = false;
        for monkey in temp_monkeys.iter() {
            if monkey.name.contains("root") {
                if monkey.right_val == monkey.left_val {
                    println!("B: {}", index);
                    target_found = true;
                    break;
                } else {
                    println!("{}", monkey.left_val - monkey.right_val);
                }
            }
        }

        if target_found {
            break;
        }
    }
}

fn resolve_monkeys(monkeys: &mut Vec<Monkey>, list: &mut Vec<Monkey>) {

    while list.len() > 0 {
        let monkey = list.pop().unwrap();

        for m in monkeys.iter_mut() {
            if m.value == 0 {
                if m.list.contains(&monkey.name) {
                    if m.left_val == 0 && m.list.get(0).unwrap().contains(&monkey.name) {
                        m.left_val = monkey.value;
                    } else if m.right_val == 0 && m.list.get(1).unwrap().contains(&monkey.name) {
                        m.right_val = monkey.value;
                    } else {
                        println!("SHOULD NOT HAPPEN!");
                    }


                    if m.right_val != 0 && m.left_val != 0 {
                        let value = calc_value(m.left_val, m.right_val, m.action.clone());
                        list.push(Monkey {name: m.name.clone(), value: value, left_val: 0, right_val: 0, list: Vec::new(), action: "".to_string()});
                        m.value = value;
                    }
                }
            }
            
        }
    }
}

fn calc_value(left: i128, right: i128, action: String) -> i128 {
    match action.as_str() {
        "+" => {return left + right;},
        "-" => {return left - right;},
        "*" => {return left * right;},
        "/" => {return left / right;},
        _ => println!("Big Error!")
    }
    println!("Return 0, very bad");
    return 0;
}
