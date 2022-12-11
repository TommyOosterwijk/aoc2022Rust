fn main() {
    println!("Day4");

    let input = include_str!("../day4.txt").split("\n");
    let result: i32 = input.clone().filter(| s | {
        let s2 = s.split(",").collect::<Vec<_>>();
        let p1 = s2.get(0).unwrap().split("-").map(|s|s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let p2 = s2.get(1).unwrap().split("-").map(|s|s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        if p1.get(0) <= p2.get(0) && p1.get(1) >= p2.get(1) {
            return true;
        } else if p2.get(0) <= p1.get(0) && p2.get(1) >= p1.get(1) {
            return true;
        }
        
        return false;
    }).map(| _s| 1).sum();

    println!("A: {}", result);

    let result_b: i32 = input.clone().filter(| s | {
        let s2 = s.split(",").collect::<Vec<_>>();
        let p1 = s2.get(0).unwrap().split("-").map(|s|s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let p2 = s2.get(1).unwrap().split("-").map(|s|s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        if p1.get(0) <= p2.get(0) && p1.get(1) >= p2.get(0) {
            return true;
        } else if p1.get(0) <= p2.get(1) && p1.get(1) >= p2.get(1) {
            return true;
        } else if p2.get(0) <= p1.get(1) && p2.get(1) >= p1.get(1) {
            return true;
        } else  if p2.get(0) <= p1.get(0) && p2.get(1) >= p1.get(0) {
            return true;
        }
        
        return false;
    }).map(| _s| 1).sum();

    println!("B: {}", result_b);

}
