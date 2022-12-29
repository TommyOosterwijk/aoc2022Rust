#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Day15!");
    let mut smallest_x = 0;
    let mut biggest_x = 0;
    let instructions = include_str!("../day15.txt");
    let mut points: Vec<Point> = Vec::new();
    let mut target_points: Vec<Point> = Vec::new();
    let mut cleared_target_points: Vec<Point> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    instructions.lines().map(|s| {
        let s = s.replace("Sensor at x=", "");
        let s = s.replace(" y=", "");
        let s = s.replace(": closest beacon is at x=", "|");
        return s;
    }).for_each(|s| {
        let result = s.split("|");
        let mut counter = 0;
        for str in result.into_iter() {
            let (x, y) = str.split_once(",").unwrap();
            let int_x: i32 = x.parse().unwrap();
            let int_y: i32 = y.parse().unwrap();

            points.push(Point { x: int_x, y: int_y });      
            if counter == 1 {
                beacons.push(Point { x: int_x, y: int_y });      
            }
            counter+=1;      
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

    for i in smallest_x..biggest_x {
        let mut beacon_found = false;
        for x in 0..beacons.len() {
            let beacon = beacons[x];
            if beacon.y == 10 && beacon.x == i.try_into().unwrap() {
                beacon_found = true;
                break;
            }
        }

        if !beacon_found {
            let point:Point = Point { x: i, y: 10 };
            target_points.push(point);
        }
    }

    for i in (0..points.len()).step_by(2) {
        let scanner_x = points[i].x;
        let scanner_y = points[i].y;
        let mut list_to_remove:Vec<usize> = Vec::new();

        let d = calculate_distance(scanner_x, scanner_y, points[i+1].x, points[i+1].y);
        for t in 0..target_points.len() {
            let target_point = target_points[t];
            let target_distance = calculate_distance(scanner_x, scanner_y, target_point.x, target_point.y);

            if target_distance <= d {
                cleared_target_points.push(target_point);
                list_to_remove.push(t);
            }
        }

        for l in (0..list_to_remove.len()).rev() {
            let index_to_remove = *list_to_remove.get(l).unwrap();
            target_points.remove(index_to_remove);
        }
    }
    println!("A: {}", cleared_target_points.len());
}

fn calculate_distance(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> i32 {
    return (start_x - end_x).abs() + (start_y - end_y).abs();
}
