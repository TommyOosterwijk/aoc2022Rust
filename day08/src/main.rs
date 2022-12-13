fn main() {
    println!("Day 8!");

    let instructions = include_str!("../day8.txt").split("\n");

    let mut forrest_b: Vec<Vec<bool>> = Vec::new();
    let mut forrest_s: Vec<Vec<i32>> = Vec::new();
    let mut forrest_v: Vec<Vec<i32>> = Vec::new();

    instructions.for_each(|s| {
        let mut forrest_row_b: Vec<bool> = Vec::new();
        let mut forrest_row_s: Vec<i32> = Vec::new();
        let mut forrest_row_v: Vec<i32> = Vec::new();

        s.chars().map(|v| v.to_digit(10).unwrap()).for_each(|char| {
            forrest_row_b.push(false);
            forrest_row_s.push(char.try_into().unwrap());
            forrest_row_v.push(1);

        });
        forrest_b.push(forrest_row_b);
        forrest_s.push(forrest_row_s);
        forrest_v.push(forrest_row_v);
    });

    let forrest_size = forrest_s.len();
    let forrest_vert_size = forrest_s.get(0).unwrap().len();

    check_trees_vertical(0, forrest_vert_size, &mut forrest_s, &mut forrest_b);
    check_trees_vertical_revert(0, forrest_vert_size, &mut forrest_s, &mut forrest_b);

    check_trees_hor(0, forrest_size, &mut forrest_s, &mut forrest_b);
    check_trees_hor_revert(0, forrest_size, &mut forrest_s, &mut forrest_b);
    let mut counter = 0;

    forrest_b.into_iter().for_each(|v| {
        for i in 0..v.len() {
            if *v.get(i).unwrap() {
                counter+=1;
            }
        }
    });
    println!("A: {}", counter);
    do_b(&mut forrest_v, &mut forrest_s);
}

fn check_trees_vertical(start: usize, end: usize, forrest_s: &mut Vec<Vec<i32>>, forrest_b: &mut Vec<Vec<bool>>) {
    for r in start..end {
        let mut highest_tree_in_row = -1;
        
        for t in 0..forrest_s.len() {
            let tree = forrest_s.get(t).unwrap().get(r).unwrap();
            if *tree > highest_tree_in_row {
                highest_tree_in_row = *tree;
                let tree_b = forrest_b.get_mut(t).unwrap().get_mut(r).unwrap();
                *tree_b = true;
            }
        }
    }
}

fn check_trees_vertical_revert(start: usize, end: usize, forrest_s: &mut Vec<Vec<i32>>, forrest_b: &mut Vec<Vec<bool>>) {
    for r in (start..end).rev() {
        let mut highest_tree_in_row = -1;
        
        for t in (0..forrest_s.len()).rev() {
            let tree = forrest_s.get(t).unwrap().get(r).unwrap();
            if *tree > highest_tree_in_row {
                highest_tree_in_row = *tree;
                let tree_b = forrest_b.get_mut(t).unwrap().get_mut(r).unwrap();
                *tree_b = true;
            }
        }
    }
}

fn check_trees_hor(start: usize, end: usize, forrest_s: &mut Vec<Vec<i32>>, forrest_b: &mut Vec<Vec<bool>>) {
    for r in start..end {
        let tree_row = forrest_s.get(r).unwrap();
        let mut highest_tree_in_row = -1;
        for t in 0..tree_row.len() {
            let tree_b = forrest_b.get_mut(r).unwrap().get_mut(t).unwrap();
            let tree = tree_row.get(t).unwrap();
            if tree > &highest_tree_in_row {
                highest_tree_in_row = *tree;
                *tree_b = true;
            }
        }
    }
}

fn check_trees_hor_revert(start: usize, end: usize, forrest_s: &mut Vec<Vec<i32>>, forrest_b: &mut Vec<Vec<bool>>) {
    for r in (start..end).rev() {
        let tree_row = forrest_s.get(r).unwrap();
        let mut highest_tree_in_row = -1;
        for t in (0..tree_row.len()).rev() {
            let tree_b = forrest_b.get_mut(r).unwrap().get_mut(t).unwrap();
            let tree = tree_row.get(t).unwrap();
            if tree > &highest_tree_in_row {
                highest_tree_in_row = *tree;
                *tree_b = true;
            }
        }
    }
}

fn do_b(forrest_v: &mut Vec<Vec<i32>>, forrest_s: &mut Vec<Vec<i32>>) {
    println!("Do b!");

    let v_len = forrest_v.len();
    for i in 0..v_len {
        let forrest_r = forrest_s.get(i).unwrap();
        let r_len = forrest_r.len();
        for r in 0..r_len {
            let tree = forrest_r.get(r).unwrap();
            let mut up_counter = 0;

            //check left
            if !(r == 0) {
                let mut temp_r = r -1;
                

                while temp_r >= 0 {
                    let temp_tree = forrest_r.get(temp_r).unwrap();
                    
                    up_counter+=1;

                    if temp_tree >= tree {
                        break;
                    }
                    if temp_r == 0 {
                        break;
                    }
                    temp_r -=1;
                }
            }
            add_value(up_counter, forrest_v, i, r);

        
            // // check right
            up_counter = 0;

            if !(r == r_len-1) {
                let mut temp_r = r + 1;

                while temp_r < r_len {
                    let temp_tree = forrest_r.get(temp_r).unwrap();
                    up_counter+=1;
                    if temp_tree >= tree {
                        break;
                    }
                    
                    if temp_r == r_len-1 {
                        break;
                    }
                    temp_r +=1;
                }
            }
            add_value(up_counter, forrest_v, i, r);


        }
    }


    let r_len = forrest_v.get(0).unwrap().len();

    for i in 0.. r_len {
        for r in 0.. v_len {
            let tree = forrest_s.get(r).unwrap().get(i).unwrap();
            let mut up_counter = 0;

            // //Going up
            if !(r == 0) {
                let mut temp_r = r -1;

                while temp_r >= 0 {
                    let temp_tree = forrest_s.get(temp_r).unwrap().get(i).unwrap();
                    up_counter+=1;
                    if tree <= temp_tree {
                        break;
                    }
                    

                    if temp_r == 0 {
                        break;
                    }
                    temp_r-=1;
                }
            }
            add_value(up_counter, forrest_v, r, i);


            up_counter = 0;

            //Going down
            if !(r == v_len-1) {
                let mut temp_r = r +1;

                while temp_r < v_len {
                    let temp_tree = forrest_s.get(temp_r).unwrap().get(i).unwrap();
                    up_counter+=1;
                    if tree <= temp_tree {
                        break;
                    }
                    

                    if temp_r == v_len-1 {
                        break;
                    }
                    temp_r+=1;
                }
            }
            add_value(up_counter, forrest_v, r, i);
        }
    }
    let mut result_b = 0;
    forrest_v.into_iter().for_each(|v|{

        v.into_iter().for_each(|val| {
            if *val > result_b {
                result_b = *val;
            }
        });

    });

    println!("B: {}", result_b);
    
}

fn add_value(value: i32, forrest_v: &mut Vec<Vec<i32>>, i: usize, r: usize) {
    let v_tree = forrest_v.get_mut(i).unwrap().get_mut(r).unwrap();
    *v_tree = *v_tree * value;   
}

