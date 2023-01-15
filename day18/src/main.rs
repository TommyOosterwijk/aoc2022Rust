use ndarray::{Array3, ArrayBase, Dim, OwnedRepr};
use scanf::sscanf;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize
}
fn main() {
    println!("Day18");

    let lines: Vec<String> = include_str!("../day18.txt").lines().map(|s|s.to_string()).collect();
    let mut grid = Array3::<i32>::zeros((20, 20, 20));
    let mut outer_water_point: Vec<Point> = Vec::new();

    lines.into_iter().for_each(|c| {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut z: usize = 0;

        sscanf!(&c, "{},{},{}", x, y, z)
            .expect("error parsing instruction");

            grid[[z,y,x]] = 1; 
    });

    let total_result = check_list_for_cube_surface(&grid, 1);
    println!("A: {}", total_result);

    for z in 0..20 {
        for y in 0..20 {
            for x in 0..20 {
                
                if grid[[z,y,x]] != 1 && (y == 0 || y == 19 || x ==0 || x == 19) {
                    outer_water_point.push(Point { x: (x), y: (y), z: (z) });
                }              
            }
        }
    }
    while outer_water_point.len() > 0 {
        let result = outer_water_point.pop().unwrap();
        let x = result.x;
        let y = result.y;
        let z = result.z;

        grid[[z,y,x]] = 2;

        if result.x > 0 {
            if grid[[z,y,x-1]] == 0 { if !outer_water_point.contains(&Point { x: (x-1), y: (y), z: (z) }) { outer_water_point.push(Point { x: (x-1), y: (y), z: (z) }); } }
        }
        if x < 19 {
            if grid[[z,y,x+1]] == 0 { if !outer_water_point.contains(&Point { x: (x+1), y: (y), z: (z) }) { outer_water_point.push(Point { x: (x+1), y: (y), z: (z) }); } }
        }
        if y > 0 {
            if grid[[z,y-1,x]] == 0 { if !outer_water_point.contains(&Point { x: (x), y: (y-1), z: (z) }) { outer_water_point.push(Point { x: (x), y: (y-1), z: (z) }); } }
        }
        if y < 19 {
            if grid[[z,y+1,x]] == 0 {  if !outer_water_point.contains(&Point { x: (x), y: (y+1), z: (z) }) { outer_water_point.push(Point { x: (x), y: (y+1), z: (z) }); } }
        }
        if z > 0 {
            if grid[[z-1,y,x]] == 0 {  if !outer_water_point.contains(&Point { x: (x), y: (y), z: (z-1) }) { outer_water_point.push(Point { x: (x), y: (y), z: (z-1) }); } }
        }
        if z < 19 {
            if grid[[z+1,y,x]] == 0 {  if !outer_water_point.contains(&Point { x: (x), y: (y), z: (z+1) }) { outer_water_point.push(Point { x: (x), y: (y), z: (z+1) }); } }
        }
    }
    let result_b = check_list_for_cube_surface(&grid, 0);
    println!("B: {}", total_result -result_b);
}

fn check_list_for_cube_surface(list: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>>, target_value: i32) -> i32 {
    let mut cube_size_total= 0;

    for z in 0..20 {
        for y in 0..20 {
            for x in 0..20 {
                let mut cube_size = 6;
                if list[[z,y,x]] == target_value {
                    if x > 0 {
                        if list[[z,y,x-1]] == target_value { cube_size-=1;}
                    }
                    if x < 19 {
                        if list[[z,y,x+1]] == target_value { cube_size-=1;}
                    }
                    if y > 0 {
                        if list[[z,y-1,x]] == target_value { cube_size-=1;}
                    }
                    if y < 19 {
                        if list[[z,y+1,x]] == target_value { cube_size-=1;}
                    }
                    if z > 0 {
                        if list[[z-1,y,x]] == target_value { cube_size-=1;}
                    }
                    if z < 19 {
                        if list[[z+1,y,x]] == target_value { cube_size-=1;}
                    }
                    cube_size_total += cube_size;
                }
            }
        }
    }
    return cube_size_total;
}