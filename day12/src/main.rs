use std::collections::HashMap;
pub struct Point {
    pub x: i32,
    pub y: i32
}

fn main() {
    println!("Day12!");
    let mut starting_location: Point = Point { x: 0, y: 0 };
    let mut map_x: i32 = 0;
    let mut map_y: i32 = 0;
    let mut step_counter: Vec<i32> = Vec::new();
    let mut found: HashMap<i32, i32> = HashMap::new();
    let mut lowest_point: Vec<usize> = Vec::new();

    let instructions = include_str!("../day12.txt");
    map_y = instructions.lines().skip(1).count().try_into().unwrap();
    instructions.lines().take(1).for_each(|v| { map_x = v.chars().count().try_into().unwrap();});
    let mut map = instructions.chars().filter(|s|!s.is_whitespace()).map(|v| {
        if v == 'S' {
            step_counter.push(-1);
            return 0;
        } else if v == 'E' {
            step_counter.push(0);
            return 27;
        }    
        step_counter.push(-1);
        return v as i32 - 96;
    }).collect::<Vec<_>>();

    for i in 0..map.len() {
        let i_counter: i32 = i.try_into().unwrap();
        let step = map.get(i).unwrap();
        if  *step == 0 {
            starting_location.x = translate_x(i_counter, map_x);            
            starting_location.y = translate_y(i_counter, map_x);
           
        } else if  *step == 27 {
            found.insert(i_counter, 0);

        }

        if *step == 1 || *step == 0 {
            lowest_point.push(i_counter.try_into().unwrap());
        }
    }
    do_a(&mut found, map_x, &mut map, &mut step_counter, map_y);
    let i:usize = (starting_location.x + (starting_location.y*map_x)).try_into().unwrap();
    println!("A: {}",step_counter.get(i).unwrap());

    let mut lowest_steps = -3;

    for key in lowest_point {
        let step = step_counter.get(key).unwrap();
        if *step > 0 && (lowest_steps > *step || lowest_steps == -3) {
            lowest_steps = *step;
        }
    }

    println!("B: {}", lowest_steps);

}

fn do_a(found: &mut HashMap<i32, i32>, map_x: i32, map: &mut Vec<i32>, step_counter: &mut Vec<i32>, map_y: i32) {

    while found.len() > 0 {
        let index = get_key_lowest_value(found);
        let i:usize = index.try_into().unwrap();
        let steps = found.get(&index).unwrap().to_owned();
        let height: i32 = map.get(i).unwrap().to_owned();
        let location: Point = Point { x: translate_x(index, map_x), y: translate_y(index, map_x) };
        check_neighbour(location.x + 1,location.y, found, map_x, map, steps, height, step_counter, map_y);
        check_neighbour(location.x -1,location.y, found, map_x, map, steps, height, step_counter, map_y);
        check_neighbour(location.x,location.y + 1, found, map_x, map, steps, height, step_counter, map_y);
        check_neighbour(location.x,location.y -1, found, map_x, map, steps, height, step_counter, map_y);
        found.remove(&index);
    }
}

fn check_neighbour(x: i32, y: i32, found: &mut HashMap<i32, i32>, map_x: i32, map: &mut Vec<i32>, step_val: i32, old_height: i32, steps: &mut Vec<i32>, map_y: i32) {
    if x >= 0 && x <= map_x-1 && y >= 0 && y <= map_y {
        let index = y*map_x + x;
        let i:usize = index.try_into().unwrap();
        if steps[i] == -1 {
            let height: i32 = *map.get(i).unwrap();

            if height >= old_height || height+1 == old_height {
                let new_step = step_val +1;
                steps[i] = new_step;
                found.insert(index, new_step);
            }
        }
    }
}

fn translate_x(value: i32, map_size: i32) -> i32{
    return value % map_size;
}

fn translate_y(value: i32, map_size: i32) -> i32{
    return value / map_size;
}

fn get_key_lowest_value(found: &mut HashMap<i32, i32>) -> i32 {
    let mut key = 0;
    let mut min_value = -1;

    for (k, v) in found.iter() {
        if *v < min_value || min_value == -1{
            key = *k;
            min_value = *v;
        }
    }
    return key;
}
