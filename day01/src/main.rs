use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    println!("Day 1");
    let mut highest_cal = 0;
    let mut current_elf_cal = 0;
    let mut vec=Vec::new();

    if let Ok(lines) = read_lines("Day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    if highest_cal < current_elf_cal {
                        highest_cal = current_elf_cal;
                    }
                    vec.push(current_elf_cal);
                    current_elf_cal = 0;
                } else {
                    let num: i32 = line.parse().unwrap();
                    current_elf_cal += num;

                }
            }
        }
    }
    vec.sort();
    let vec_len = vec.len();

    let highest1 = vec.get(vec_len-1).unwrap();
    let highest2 = vec.get(vec_len-2).unwrap();
    let highest3 = vec.get(vec_len-3).unwrap();

    println!("A: {}", highest_cal);
    println!("B: {}", highest1+highest2+highest3);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}