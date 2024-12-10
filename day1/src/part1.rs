pub fn solve(lines: &Vec<String>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let len = parts.len();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[len - 1].parse().unwrap();
        list1.push(x);
        list2.push(y);
    }
    list1.sort();
    list2.sort();
    let len = list1.len();
    let mut diffsum = 0;
    for i in 0..len {
        diffsum += (list1[i] - list2[i]).abs();
    }
    println!("{}", diffsum);
}