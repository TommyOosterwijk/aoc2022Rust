use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Day9!");

    let instructions = include_str!("../day9.txt").split("\n");
    let mut head: Point = Point { x: 0, y: 0 };
    let mut tail: Point = Point { x: 0, y: 0 };
    let mut tail_coordinates = HashMap::new();
    let mut tail_coordinates_b = HashMap::new();
    let mut points: Vec<Point> = Vec::new();
    let point_size = 10;
    for _ in 0..point_size {
        points.push(Point { x: 0, y: 0 });
    }
    tail_coordinates.insert(format!("x{}y{}", 0, 0), true);
    tail_coordinates_b.insert(format!("x{}y{}", 0, 0), true);

    instructions.clone().for_each(|s| {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        let dir = split.get(0).unwrap();
        let steps_s = split.get(1).unwrap().parse::<i32>().unwrap();

        for _ in 0..steps_s {
            if dir.contains("U") {
                head.y+=1;                
            } else if dir.contains("D") {
                head.y-=1;
            } else if dir.contains("R") {
                head.x+=1;            
            }else if dir.contains("L") {
                head.x-=1;
            }

            let last_tail = true;
            move_tail_node(&mut head, &mut tail, &mut tail_coordinates, last_tail);  
        
        }
    });
    println!("A: {}", tail_coordinates.len());

    instructions.clone().for_each(|s| {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        let dir = split.get(0).unwrap();
        let steps_s = split.get(1).unwrap().parse::<i32>().unwrap();
        for _ in 0..steps_s {
            if dir.contains("U") {
                points[0].y+=1;                
            } else if dir.contains("D") {
                points[0].y-=1;
            } else if dir.contains("R") {
                points[0].x+=1;            
            }else if dir.contains("L") {
                points[0].x-=1;
            }
            
            for i in 1..point_size {
                let mut last_tail = false;
                
                if i == 9 {
                    last_tail = true;
                }

                let mut head = Point { x: points.get(i-1).unwrap().x, y: points.get(i-1).unwrap().y };
                let mut tail = &mut points[i];

                move_tail_node(&mut head, &mut tail, &mut tail_coordinates_b, last_tail);
            }
           
        }
    });
    println!("B: {}", tail_coordinates_b.len());
}

fn move_tail_node(head: &mut Point, tail: &mut Point, map: &mut HashMap<String, bool>, is_last_tail: bool) {
    

    if head.x == tail.x {
        if head.y - tail.y == 2 {
            tail.y+=1;
        } else if head.y - tail.y == -2 {
            tail.y -=1;
        }
    } else if head.y == tail.y {
        if head.x - tail.x == 2 {
            tail.x+=1;
        } else if head.x - tail.x == -2 {
            tail.x -=1;
        }
    } else {
        if (head.y - tail.y).abs() == 2 && (head.x - tail.x).abs() == 2 {
            // the most distance between a head and tail node.
            // move 1,1 towards head.
            if head.y > tail.y {
                tail.y+=1;
            } else {
                tail.y-=1;
            }

            if head.x > tail.x {
                tail.x+=1;
            } else {
                tail.x-=1;
            }
        }
    
        if(head.y - tail.y).abs() == 2 && (head.x - tail.x).abs() == 1 {
            tail.x = head.x;
            if head.y > tail.y { tail.y = head.y-1} else { tail.y = head.y+1 }
        }

        if (head.x - tail.x).abs() == 2 && (head.y - tail.y).abs() == 1 {
            tail.y = head.y;
            if head.x > tail.x { tail.x = head.x-1} else { tail.x = head.x+1 }
        }
    }

    if is_last_tail {
        let s = format!("x{}y{}", tail.x, tail.y);
        map.insert(s, true);
    }
}