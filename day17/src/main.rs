fn main() {
    println!("Day17");
    let instructions: Vec<char> = include_str!("../day17.txt").chars().collect();
    let empty_line = Vec::from(['.','.','.','.','.','.', '.']);
    let mut chamber: Vec<Vec<char>> = Vec::from([Vec::from(['-','-','-','-','-','-', '-'])]);    
    let line:Vec<Vec<char>> = Vec::from([Vec::from(['.','.','@','@','@','@','.'])]);
    let cross: Vec<Vec<char>> = Vec::from([Vec::from(['.','.','.','@','.','.', '.']), Vec::from(['.','.','@','@','@','.', '.']), Vec::from(['.','.','.','@','.','.', '.'])]);
    let l: Vec<Vec<char>> = Vec::from([Vec::from(['.','.','@','@','@','.', '.']), Vec::from(['.','.','.','.','@','.', '.']), Vec::from(['.','.','.','.','@','.', '.'])]);
    let line_vertical: Vec<Vec<char>> = Vec::from([Vec::from(['.','.','@','.','.','.', '.']), Vec::from(['.','.','@','.','.','.', '.']), Vec::from(['.','.','@','.','.','.', '.']), Vec::from(['.','.','@','.','.','.', '.'])]);
    let block: Vec<Vec<char>> = Vec::from([Vec::from(['.','.','@','@','.','.', '.']), Vec::from(['.','.','@','@','.','.', '.'])]);
 
    let mut game_loop = 0;
    let mut insutrction_index = 0;
      
    let max_x = 7;
    let mut max_y = chamber.len();

    let mut end_result:Vec<Vec<char>> = Vec::new();
    let mut first_time_ending = true;
    let mut old_max_y = 0;
    //print_chamber(max_x, max_y, &chamber);

    let mut test:Vec<usize> = Vec::new();
    let mut test2:Vec<usize> = Vec::new();
    let mut test2_bool = false;

    for counter in 0..50000 {
       
        let mut target: Vec<Vec<char>> = Vec::new();
        let mut target_start_x = 2;
        let mut target_end_x = 0;

        chamber.push(empty_line.clone());
        chamber.push(empty_line.clone());
        chamber.push(empty_line.clone());

        match game_loop {
            0 => {chamber.append(&mut line.clone()); target.append(&mut line.clone()); target_end_x = 5;},
            1 => {chamber.append(&mut cross.clone()); target.append(&mut cross.clone()); target_end_x = 4;},
            2 => {chamber.append(&mut l.clone()); target.append(&mut l.clone()); target_end_x = 4;},
            3 => {chamber.append(&mut line_vertical.clone()); target.append(&mut line_vertical.clone()); target_end_x = 2;},
            4 => {chamber.append(&mut block.clone()); target.append(&mut block.clone()); target_end_x = 3;},
            _ => println!("Warning!!")
        }       

        max_y = chamber.len();
        let mut target_start_y = max_y;

        //print_chamber(max_x, max_y, &chamber);
        let mut object_collision = false;   

        while !object_collision {  
            if instructions.get(insutrction_index).unwrap() == &'<' {
                if target_start_x > 0 {
                    let mut side_collision = false;
        
                    if game_loop == 1 {
                        if chamber.get(target_start_y-1).unwrap().get(target_start_x).unwrap() != &'.' || 
                        chamber.get(target_start_y-2).unwrap().get(target_start_x-1).unwrap() != &'.' || 
                        chamber.get(target_start_y-3).unwrap().get(target_start_x).unwrap() != &'.' {
                            side_collision = true;
                        }
                    } else if game_loop == 2 {
                        if chamber.get(target_start_y-1).unwrap().get(target_start_x+1).unwrap() != &'.' || 
                        chamber.get(target_start_y-2).unwrap().get(target_start_x+1).unwrap() != &'.' || 
                        chamber.get(target_start_y-3).unwrap().get(target_start_x-1).unwrap() != &'.' {
                            side_collision = true;
                        }
                    }
                    else {
                        for target_y in target_start_y - target.len().. target_start_y {
                            if chamber.get(target_y).unwrap().get(target_start_x).unwrap() != &'.' {
                                if chamber.get(target_y).unwrap().get(target_start_x-1).unwrap() != &'.' {
                                    side_collision = true;
                                    break;
                                }
                            }
                        }
                    }
        
                    if !side_collision {
                        for target_y in target_start_y - target.len().. target_start_y {
                            let lookup_index = target_y + target.len() - target_start_y;
                            for index in target_start_x..(target_end_x+1) {
                                if target.get(lookup_index).unwrap().get(index +2 - target_start_x).unwrap() != &'.' {
                                    let next_target = *chamber.get(target_y).unwrap().get(index).unwrap();
                                    let mut target = chamber.get_mut(target_y).unwrap().get_mut(index-1).unwrap();
                                    *target = next_target;
                                    target = chamber.get_mut(target_y).unwrap().get_mut(index).unwrap();
                                    *target = '.';
                                }
                            }
                        }
                        target_end_x-=1;
                        target_start_x-=1;
                    }                    
                }
            } else {
                if target_end_x < 6 {
                    let mut side_collision = false;
        
                    if game_loop == 1 {
                        if chamber.get(target_start_y-1).unwrap().get(target_end_x).unwrap() != &'.' || 
                        chamber.get(target_start_y-2).unwrap().get(target_end_x+1).unwrap() != &'.' || 
                        chamber.get(target_start_y-3).unwrap().get(target_end_x).unwrap() != &'.' {
                            side_collision = true;
                        }
                    } else {
                        for target_y in target_start_y - target.len().. target_start_y {
                            if chamber.get(target_y).unwrap().get(target_end_x).unwrap() != &'.' {
                                if chamber.get(target_y).unwrap().get(target_end_x+1).unwrap() != &'.' {
                                    side_collision = true;
                                    break;
                                }
                            }
                        }
                    }       

                    if !side_collision {
                        for target_y in target_start_y - target.len().. target_start_y {
                            let lookup_index = target_y + target.len() - target_start_y;
                            for index in (target_start_x..(target_end_x+1)).rev() {
                                let x_size = target_end_x - target_start_x;
                                if target.get(lookup_index).unwrap().get(index + 2 + x_size - target_end_x).unwrap() != &'.' {
                                    let next_target = *chamber.get(target_y).unwrap().get(index).unwrap();
                                    let mut target = chamber.get_mut(target_y).unwrap().get_mut(index+1).unwrap();
                                    *target = next_target;
                                    target = chamber.get_mut(target_y).unwrap().get_mut(index).unwrap();
                                    *target = '.';
                                }
                            }
                        }
                        target_end_x+=1;
                        target_start_x+=1;
                    }  
                }
            } 
        
            //print_chamber(max_x, max_y, &chamber);
            if !object_collision {                
                
        
                if game_loop == 1 {
                    if chamber.get(target_start_y - target.len()).unwrap().get(target_start_x).unwrap() != &'.' || 
                    chamber.get(target_start_y-1 - target.len()).unwrap().get(target_start_x+1).unwrap() != &'.' || 
                    chamber.get(target_start_y - target.len()).unwrap().get(target_start_x+2).unwrap() != &'.' {
                        object_collision = true;
                    }
                } else {
                    let target_y =  target_start_y-1 - target.len();
                    for index in target_start_x..(target_end_x+1) {
                        if chamber.get(target_y+1).unwrap().get(index).unwrap() != &'.' {
                            if chamber.get(target_y).unwrap().get(index).unwrap() != &'.' {
                                object_collision = true;
                                break;
                            }
                        }
                    }
                }
        
                if !object_collision {


                   
                    for index in target_start_x..(target_end_x+1) {
                        for target_y in target_start_y - target.len().. target_start_y {
                            let lookup_index = target_y + target.len() - target_start_y;

                            if target.get(lookup_index).unwrap().get(index +2 - target_start_x).unwrap() != &'.' {
                                let next_target = *chamber.get(target_y).unwrap().get(index).unwrap();
                                let mut target = chamber.get_mut(target_y-1).unwrap().get_mut(index).unwrap();
                                *target = next_target;
                                target = chamber.get_mut(target_y).unwrap().get_mut(index).unwrap();
                                *target = '.';
                            }
                        }
                    }
                    target_start_y-=1;
                    let mut elements = false;
        
                    for i in 0..7 {
                        if chamber.get(max_y-1).unwrap().get(i).unwrap() != &'.' {
                            elements = true;
                            break;
                        }
                    }
                    if !elements {
                        chamber.pop();
                        max_y-=1;
                    }                    
                }                
            }            
            //print_chamber(max_x, max_y, &chamber);

            
            insutrction_index+=1;
            if insutrction_index == instructions.len() {
                
                

                if first_time_ending {

                    for y in (max_y-20)..max_y {
                        end_result.push(chamber.get_mut(y).unwrap().clone());
                    }
                    first_time_ending = false;
                }

                insutrction_index = 0;
            }

            if !first_time_ending {
                if is_same(&end_result, &chamber, 20) {
                    //println!("Party!! Counter = {}", counter);
                }
            }                    
        }   
        game_loop+=1;
        if game_loop == 5 {
            game_loop = 0;
        }


        if test2_bool {
            test2.push(max_y-1 - old_max_y);
        }
        //println!("{}",max_y-1 - old_max_y);
        test.insert(0, max_y-1 - old_max_y);

        let string = test.clone().into_iter().map(|i| i.to_string()).collect::<String>();
        if string == "223310310023031023310033" {
            println!("Obj Counter = {}", counter);

            if test2_bool {
                //test2_bool = false;
            }
            test2_bool = true;

        
        }

        old_max_y = max_y -1;

        if test.len() > 23 {
            test.pop();
        }

    }

    for i in 980..test2.len() {
       // println!("{}",test2[i]);
    }

        
     println!("A: {}", max_y-1);
}

fn test() {
    let mut start = 26;
    let up = 53;

    let diff = 2000 - 50;
    let devide = diff / 200;

    println!("{}", (up*devide) + start);
}

fn is_same(first_sub_chamber: &Vec<Vec<char>>, chamber: &Vec<Vec<char>>, y_size: usize) -> bool {
    let first_sub_len = first_sub_chamber.len();
    let chamber_len = chamber.len();
    for y in 0..y_size {
        if first_sub_chamber.get(first_sub_len - y) != chamber.get(chamber_len - y) {
            return false;
        }
    }

    return true;
}
        
fn print_chamber(max_x: usize, max_y: usize, chamber: &Vec<Vec<char>>) {
    println!();
    for y in (max_y-15..max_y).rev() {
        for x in 0..max_x {
           print!("{}", chamber.get(y).unwrap().get(x).unwrap());
        }
        println!();
    }
}

