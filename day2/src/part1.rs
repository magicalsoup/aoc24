pub fn solve(lines: &Vec<String>) {
    let mut cnt = 0;
    for line in lines {
        let list = get_list(line);
        if is_safe(list) {
            cnt += 1;
        } 
    }
    println!("{}", cnt);
}

fn get_list(line: &String) -> Vec<i32> {
    let parts: Vec<&str> = line.split(" ").collect();
    let mut list: Vec<i32> = Vec::new();
    for part in parts {
        let x: i32 = part.parse().unwrap();
        list.push(x);
    }
    list
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut state = 0; // null state, 1 = asc, 2 = desc
    let len = levels.len();
    for i in 1..len {
        let diff = (levels[i] - levels[i-1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if levels[i] > levels[i-1] {
            if state == 0 {
                state = 1;
            } else if state != 1{
                return false;
            }
        } 
        if levels[i] < levels[i-1] {
            if state == 0 {
                state = 2;
            } else if state != 2{
                return false;
            }
        }
    }
    true
}


// for brute force testing purposes
// fn remove_one_is_safe(levels: Vec<i32>) -> bool {
//     let len = levels.len();
//     for i in 0..len {
//         let mut cpy = levels.clone();
//         cpy.remove(i);
//         if is_safe(cpy) {
//             return true;
//         }
//     }
//     false
// }