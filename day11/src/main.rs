use std::collections::HashMap;

struct Monkey {
    items_list: Vec<u128>,
    operation:  String,
    devided_by: u128,
    give_to: HashMap<bool, usize>,
}

fn main() {
    println!("Day11");

    let mut monkey_list: Vec<Monkey> = Vec::new();

    monkey_list.push(Monkey { items_list: Vec::from([92, 73, 86, 83, 65, 51, 55, 93]), operation: "* 5".to_string(), devided_by: 11, give_to:  HashMap::from([(true, 3),(false, 4)])});
    monkey_list.push(Monkey { items_list: Vec::from([99, 67, 62, 61, 59, 98]), operation: "*".to_string(), devided_by: 2, give_to:  HashMap::from([(true, 6),(false, 7)])});
    monkey_list.push(Monkey { items_list: Vec::from([81, 89, 56, 61, 99]), operation: "* 7".to_string(), devided_by: 5, give_to:  HashMap::from([(true, 1),(false, 5)])});
    monkey_list.push(Monkey { items_list: Vec::from([97, 74, 68]), operation: "1".to_string(), devided_by: 17, give_to:  HashMap::from([(true, 2),(false, 5)])});


    monkey_list.push(Monkey { items_list: Vec::from([78, 73]), operation: "3".to_string(), devided_by: 19, give_to:  HashMap::from([(true, 2),(false, 3)])});
    monkey_list.push(Monkey { items_list: Vec::from([50]), operation: "5".to_string(), devided_by: 7, give_to:  HashMap::from([(true, 1),(false, 6)])});
    monkey_list.push(Monkey { items_list: Vec::from([95, 88, 53, 75]), operation: "8".to_string(), devided_by: 3, give_to:  HashMap::from([(true, 0),(false, 7)])});
    monkey_list.push(Monkey { items_list: Vec::from([50, 77, 98, 85, 94, 56, 89]), operation: "2".to_string(), devided_by: 13, give_to:  HashMap::from([(true, 4),(false, 0)])});


    let mut place_holders: Vec<Vec<u128>> = Vec::new();
    let mut monkey_inspect_counter: Vec<u128> = Vec::new();

    for i in 0.. monkey_list.len() {
        let v: Vec<u128> = Vec::new();
        place_holders.push(v);
        monkey_inspect_counter.push(0);
    }

    for _ in 0..10000 {
        for m in 0..monkey_list.len() {
            let monkey = monkey_list.get_mut(m).unwrap();

            for i in 0..monkey.items_list.len() {
                let val = monkey.items_list.get_mut(i).unwrap();
                let test = &monkey.operation;
                let result:u128 = do_operation(val, test);
                let index = *monkey.give_to.get(&(result % monkey.devided_by == 0)).unwrap();
                place_holders.get_mut(index).unwrap().push(result);
                monkey_inspect_counter[m]+=1;

            }

            for _ in 0..monkey.items_list.len() {
                monkey.items_list.pop();
            }

            for i in 0..place_holders.len() {
                let holder = &mut place_holders[i];

                while holder.len() > 0 {
                    monkey_list.get_mut(i).unwrap().items_list.push(holder.pop().unwrap());
                }
            }
        }
    }
    monkey_inspect_counter.sort();

    println!("B: {}", (monkey_inspect_counter.get(monkey_inspect_counter.len()-1).unwrap() * monkey_inspect_counter.get(monkey_inspect_counter.len()-2).unwrap()));
}

fn do_operation(val: &u128, operation: &String) -> u128{
    let mut result: u128 = 0;

    if operation.contains("*") {
        if operation.len() == 1 {
            result = val * val;
       } else {
        let (_, split2) = operation.split_at(2);
            result = val * split2.parse::<u128>().unwrap();
       }
    } else {
        result = val + operation.parse::<u128>().unwrap();
    }

    return result % 9699690;
}
