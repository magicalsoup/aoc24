use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) {
    let mut cnt = HashMap::new();

    let mut list1: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let len = parts.len();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[len - 1].parse().unwrap();
        list1.push(x);
        let count = cnt.entry(y).or_insert(0);
        *count += 1;
    }

    let mut sim = 0;

    for x in list1 {
        sim += x * cnt.get(&x).copied().unwrap_or(0);
    }
    println!("{}", sim);
}