#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Copy, Clone, Debug)]
struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn main() {
    println!("Day15!");    
    let mut smallest_x = 0;
    let mut biggest_x = 0;
    let instructions = include_str!("../day15.txt");

    let mut points: Vec<Point> = Vec::new();

    instructions.lines().map(|s| {
        let mut s = s.replace("Sensor at x=", "");
        s = s.replace(" y=", "");
        s = s.replace(": closest beacon is at x=", "|");
        return s;
    }).for_each(|s| {
        let result = s.split("|");
        for str in result.into_iter() {
            let (x, y) = str.split_once(",").unwrap();
            let int_x: i32 = x.parse().unwrap();
            let int_y: i32 = y.parse().unwrap();

            points.push(Point { x: int_x, y: int_y });      
        }
    });

    for i in (0..points.len()).step_by(2) {
        let scanner_x = points[i].x;
        let scanner_y = points[i].y;

        let d = calculate_distance(scanner_x, scanner_y, points[i+1].x, points[i+1].y);
        
        if (scanner_x - d) < smallest_x {
            smallest_x = scanner_x - d;
        }

        if (scanner_x + d) > biggest_x {
            biggest_x = scanner_x + d;
        }
    }

    for y_target in 0..4000000 {

        let mut target_lines: Vec<Line> = Vec::new();

        target_lines.push(Line { start_x: (smallest_x), start_y: (y_target), end_x: (biggest_x), end_y: (y_target) });

        for i in (1..points.len()).step_by(2) {
            let line = points.get(i).unwrap();
            if line.y == y_target {
                let mut remove_list: Vec<usize> = Vec::new();
                let mut add_list: Vec<Line> = Vec::new();

                for x in 0..target_lines.len() {
                    let target = target_lines.get_mut(x).unwrap();

                    if target.start_x <= line.x && target.end_x >= line.x {
                        if line.x +1 <= target.end_x {
                            let new_l:Line = Line { start_x: (line.x +1), start_y: (y_target), end_x: (target.end_x), end_y: (y_target) };
                            add_list.push(new_l);
                        }
                        target.end_x = line.x - 1;

                        if target.end_x < target.start_x {
                            remove_list.push(x);
                        }
                    }            
                }

                for i in (0..remove_list.len()).rev() {
                    target_lines.remove(*remove_list.get(i).unwrap());
                }

                for i in 0..add_list.len() {
                    target_lines.push(*add_list.get(i).unwrap());
                }
            }
        }
        
        let mut start_a = 0;
        for i in 0..target_lines.len() {
            let line = target_lines.get(i).unwrap();
            start_a += (line.end_x - line.start_x) +1 ;
        }

        for i in (0..points.len()).step_by(2) {
            let scanner_x = points[i].x;
            let scanner_y = points[i].y;

            let d = calculate_distance(scanner_x, scanner_y, points[i+1].x, points[i+1].y);
            let mut remove_list: Vec<usize> = Vec::new();
            let mut add_list: Vec<Line> = Vec::new();
            for index in 0..target_lines.len() {
                let mut t = target_lines.get_mut(index).unwrap();

                let d_to_y = (scanner_y-t.start_y).abs();

                if d_to_y < d {
                    let steps = d-d_to_y;
                    let end_x = scanner_x+steps;
                    let start_x = scanner_x-steps;
                    
                    if start_x <= t.start_x && end_x >= t.end_x {
                        remove_list.push(index);
                    } else if start_x <= t.start_x && end_x >= t.start_x {
                        t.start_x = end_x + 1;
                    } else if end_x >= t.end_x && start_x <= t.end_x {
                        t.end_x = start_x -1;
                    } else if t.start_x < start_x && t.end_x > end_x {              
                        let line: Line = Line { start_x: end_x+1, start_y: (y_target), end_x: (t.end_x), end_y: (y_target) };
                        add_list.push(line);
                        t.end_x = start_x-1;
                    }            
                } 
            }

            for i in (0..remove_list.len()).rev() {
                target_lines.remove(*remove_list.get(i).unwrap());
            }

            for i in 0..add_list.len() {
                target_lines.push(*add_list.get(i).unwrap());
            }
        }
        let mut a = 0;
        let mut b:i128 = 0;
        for i in 0..target_lines.len() {
            let line = target_lines.get(i).unwrap();
            a += (line.end_x - line.start_x) +1 ;

            if line.start_x == line.end_x && line.start_y == line.end_y && line.start_x >= 0 && line.end_x <=4000000 && line.start_y >= 0 && line.end_y <=4000000 {
                let x:i128 = line.start_x.try_into().unwrap();
                let y:i128 = line.start_y.try_into().unwrap();
                b = (4000000 * x) + y;
            }
        }
        if 2000000 == y_target {
            println!("A: {}", start_a - a);
        }

        if b != 0 {
            println!("B: {}", b);
        }
    }
}

fn calculate_distance(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> i32 {
    return (start_x - end_x).abs() + (start_y - end_y).abs();
}
