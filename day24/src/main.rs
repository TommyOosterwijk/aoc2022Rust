use ndarray::{Array2};

#[derive(Copy, Clone, Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    direction: i32
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Me {
    x: usize,
    y: usize
}

fn main() {
    println!("Day24!");
    let board: Vec<&str> = include_str!("../day24.txt").lines().collect();

    let mut blizzards: Vec<Blizzard> = Vec::new();
    let mut multi_me: Vec<Me> = Vec::new();

    let board_y = board.len();
    let mut board_x = 0;

    board.clone().into_iter().for_each(|s| {
        if board_x < s.len() {
            board_x = s.len();
        }
    });

    let mut temp_y = 0;

    let mut grid = Array2::<i32>::zeros((board_y, board_x));

    board.clone().into_iter().for_each(|s| {
        let mut temp_x = 0;
        for ch in s.chars() {
            if ch == '#' {
                grid[[temp_y, temp_x]] = 0;
            } else if ch == '.' {
                grid[[temp_y, temp_x]] = 1;
            } else {
                grid[[temp_y, temp_x]] = 2;
                let mut temp_blizz: Blizzard = Blizzard { x: (temp_x), y: (temp_y), direction: (0) };
                if ch == '^' {
                    temp_blizz.direction = 0;
                } else if ch == 'v' {
                    temp_blizz.direction = 2;
                } else if ch == '>' {
                    temp_blizz.direction = 1;
                } else if ch == '<' {
                    temp_blizz.direction = 3;
                }
                blizzards.push(temp_blizz);
            }
            temp_x+=1;
        }
        temp_y+=1;
    });

    let me:Me = Me { x: (1), y: (0) };
    multi_me.push(me);

    let end_x = board_x-2;
    let end_y = board_y-1;

    grid[[me.y, me.x]] = 3;
    //grid[[end_y, end_x]] = 4;

    //print_grid(&grid, board_y, board_x);
    let mut counter = 0;
    let mut a_found = false;
    let mut b_1found = false;

    while true {

        let mut temp_multi_me: Vec<Me> = Vec::new();


        for i in 0..blizzards.len() {
            let target_blizz = blizzards.get_mut(i).unwrap();         
            grid[[target_blizz.y, target_blizz.x]] = 1;
        }

        for i in 0..multi_me.len() {
            let target_me = multi_me.get_mut(i).unwrap();         
            grid[[target_me.y, target_me.x]] = 1;
        }

        //print_grid(&grid, board_y, board_x);

        for i in 0..blizzards.len() {
            let target_blizz = blizzards.get_mut(i).unwrap();
            
            if target_blizz.direction == 0 {
                target_blizz.y-=1;
                if target_blizz.y == 0 {
                    target_blizz.y = board_y-2;
                }
            } else if target_blizz.direction == 1 {
                target_blizz.x+=1;
                if target_blizz.x == board_x-1 {
                    target_blizz.x = 1;
                }
            } else if target_blizz.direction == 2 {
                target_blizz.y+=1;
                if target_blizz.y == board_y-1 {
                    target_blizz.y = 1;
                }
            } else if target_blizz.direction == 3 {
                target_blizz.x-=1;
                if target_blizz.x == 0 {
                    target_blizz.x = board_x-2;
                }
            }
            grid[[target_blizz.y, target_blizz.x]] = 2;
        }
        
        for i in 0..multi_me.len() {
            let target_me = multi_me.get_mut(i).unwrap();   

            if target_me.y > 0 && grid[[target_me.y-1, target_me.x]] == 1 {
                let temp_me:Me = Me { x: (target_me.x), y: (target_me.y-1) };
                temp_multi_me.push(temp_me);
                grid[[temp_me.y, temp_me.x]] = 3;
            }

            if grid[[target_me.y, target_me.x+1]] == 1 {
                let temp_me:Me = Me { x: (target_me.x+1), y: (target_me.y) };
                temp_multi_me.push(temp_me);
                grid[[temp_me.y, temp_me.x]] = 3;
            }

            if target_me.y < board_y-1 && grid[[target_me.y+1, target_me.x]] == 1 {
                let temp_me:Me = Me { x: (target_me.x), y: (target_me.y+1) };
                temp_multi_me.push(temp_me);
                grid[[temp_me.y, temp_me.x]] = 3;
            }

            if grid[[target_me.y, target_me.x-1]] == 1 {
                let temp_me:Me = Me { x: (target_me.x-1), y: (target_me.y) };
                temp_multi_me.push(temp_me);
                grid[[temp_me.y, temp_me.x]] = 3;
            }
            

            if  grid[[target_me.y, target_me.x]] == 1 {
                let temp_me:Me = Me { x: (target_me.x), y: (target_me.y) };
                temp_multi_me.push(temp_me);
                grid[[temp_me.y, temp_me.x]] = 3;
            }
        } 
                
        //print_grid(&grid, board_y, board_x);
        multi_me = temp_multi_me;
        counter+=1;

        if grid[[end_y, end_x]] == 3 && !a_found {
            for i in 0..multi_me.len() {
                let target_me = multi_me.get_mut(i).unwrap();         
                grid[[target_me.y, target_me.x]] = 1;
            }
            
            println!("A: {}", counter);
            multi_me = Vec::new();
            let me:Me = Me { x: (end_x), y: (end_y) };
            multi_me.push(me);
            a_found = true;
            print_grid(&grid, board_y, board_x);
        }

        if grid[[0, 1]] == 3 && a_found && !b_1found {
            for i in 0..multi_me.len() {
                let target_me = multi_me.get_mut(i).unwrap();         
                grid[[target_me.y, target_me.x]] = 1;
            }
            
            multi_me = Vec::new();
            let me:Me = Me { x: (1), y: (0) };
            multi_me.push(me);
            b_1found = true;
            print_grid(&grid, board_y, board_x);
        }

        if grid[[end_y, end_x]] == 3 && a_found && b_1found { 
            println!("B: {}", counter);
            break;
        }

    }
        print_grid(&grid, board_y, board_x);


}

fn print_grid(grid: &Array2<i32>, max_y: usize, max_x: usize) {
    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", grid[[y,x]]);    
        }
        println!();
    }
    println!();
}