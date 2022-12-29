fn main() {
    println!("Day 14!");
    let mut min_x: usize = 500; let mut min_y = 0; let mut max_x = 500; let mut max_y = 0;
    let instructions = include_str!("../day14.txt");

    instructions.lines().for_each(| s| {
        let result = s.split(" -> ").map(|s| s.split(","));
        for str in result.into_iter() {
            let mut index = 0;
            for s in str.into_iter() {
                let value:usize = s.parse().unwrap();
                if index == 0 {                    
                    if value < min_x {
                        min_x = value;
                    }
                    if value > max_x {
                        max_x = value;
                    }
                } else {
                    if value < min_y {
                        min_y = value;
                    }
                    if value > max_y {
                        max_y = value;
                    }                }
                index+=1;
            }
        }
    });
    
    //cheat for B
    max_x+=200;
    min_x-=200;


    let x_row: usize = ((max_x-min_x) +1).try_into().unwrap();
    let y_row: usize = (max_y +3).try_into().unwrap();
    let mut grid = vec![vec!['.'; x_row]; y_row];

    //cheat for B
    for i in 0.. x_row {
        grid[y_row-1][i] = '#';
    }

    instructions.lines().for_each(| s| {
        let mut temp_vec: Vec<String> = Vec::new();

        s.split(" -> ").for_each(|s| {
            let new_s = s.replace(",", "");
            temp_vec.push(new_s);
        });

        for i in 0..temp_vec.len()-1 {
            let (start_x, start_y) = temp_vec.get(i).unwrap().split_at(3);
            let (end_x, end_y) = temp_vec.get(i+1).unwrap().split_at(3);

            let i_start_x: usize = start_x.parse().unwrap();
            let i_start_y: usize = start_y.parse().unwrap();
            let i_end_x: usize = end_x.parse().unwrap();
            let i_end_y: usize = end_y.parse().unwrap();
            
            if i_start_y == i_end_y {
                if i_start_x > i_end_x {
                    for x in i_end_x..i_start_x+1 {
                        grid[i_start_y][x-min_x] = '#';
                    }
                } else {
                    for x in i_start_x..i_end_x+1 {
                        grid[i_start_y][x-min_x] = '#';
                    }
                }
            } else {
                if i_start_y > i_end_y {
                    for y in i_end_y..i_start_y+1 {
                        grid[y][i_start_x-min_x] = '#';
                    }
                } else {
                    for y in i_start_y..i_end_y+1 {
                        grid[y][i_start_x-min_x] = '#';
                    }
                }
            }

        }   
    });

    //print_board(y_row, x_row, &grid);
    let mut sand_dropped = 0;
    let mut game_state = true;

    while game_state {
        
        let mut sand_x: usize = 500 - min_x;
        let mut sand_y: usize = 0;
        let mut sand_drop = true;

        while sand_drop {
            // B
            if sand_x == (500 - min_x) && sand_y == 0 {
                if grid[sand_y][sand_x] != '.' {
                    game_state = false;
                    break;
                }
            } 
            
            if sand_y >= y_row {
                game_state = false;
                break;
            }

            let str = grid[sand_y][sand_x];
            if str == '.' {
                sand_y+=1;
            } else {
                let i_sand_x: i32 = sand_x.try_into().unwrap();
                if i_sand_x-1 >= 0 {
                    if grid[sand_y][sand_x-1] == '.' {
                        sand_x-=1;
                    } else { 
                        if sand_x +1 <= max_x {
                            if grid[sand_y][sand_x+1] == '.' {
                            sand_x+=1;
                            } else {
                                sand_drop = false;
                            }
                        } else {
                            game_state = false;
                            break;
                        }
                    }
                } else {
                    game_state = false;
                    break;
                }
            }
        }
        if game_state != false {
            grid[sand_y-1][sand_x] = 'o';
            sand_dropped+=1;
        }
    }
    println!("A: {}", sand_dropped);    
}

fn print_board(y_row: usize, x_row: usize, grid: &Vec<Vec<char>> ) {
    for y in 0..y_row {
        for x in 0..x_row {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}