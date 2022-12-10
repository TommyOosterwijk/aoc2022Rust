use std::collections::HashMap;

fn main() {
    print!("Day 2!");
    let rps_score = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6)
    ]);

    let rps_refactor = HashMap::from([
        ("A X", "A Z"),
        ("A Y", "A X"),
        ("A Z", "A Y"),
        ("B X", "B X"),
        ("B Y", "B Y"),
        ("B Z", "B Z"),
        ("C X", "C Y"),
        ("C Y", "C Z"),
        ("C Z", "C X")
    ]);

    let input = include_str!("../day2.txt");

    let sum: i32 = input.split("\n").map(| i| {
        return rps_score.get(i).unwrap();
    }).sum();

    println!("A: {}", sum.to_string());

    let sum_b: i32 = input.split("\n").map(|s| {
        return rps_refactor.get(s).unwrap();
    }).map(| i| {
        return rps_score.get(i).unwrap();
    }).sum();

    println!("B: {}", sum_b.to_string());

}
