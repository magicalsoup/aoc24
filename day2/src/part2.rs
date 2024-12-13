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

// assume len > 2
fn can_remove(levels: &Vec<i32>, i: &mut usize, state: &mut i32) -> bool {
    // x, y, i, w, z
    // problem found with y and i
    // try removing either y or i
    // y = i - 1

    let len = levels.len();
    let y: usize = *i - 1;
    
    if *i == len - 1 { // we can always remove the last element
        *i += 1;
        return true; 
    }

    // otherwise, we have to try removing y or i
    // try removing i
    let w: usize = *i + 1;
    let diff_yw: i32 = (levels[y] - levels[w]).abs();
    let ord_yw: i32 = if levels[y] > levels[w] { 2 } else { 1 };

    if diff_yw >= 1 && diff_yw <= 3 && (ord_yw == *state || *state == 0) {
        *i += 2;
        *state = ord_yw;
        return true;
    }

    if y == 0 {
        *state = 0;
        *i += 1;
        return true;
    }

    // try removing y
    let x: usize = y - 1;
    let diff_xi: i32 = (levels[x] - levels[*i]).abs();
    let ord_xi: i32 = if levels[x] > levels[*i] { 2 } else { 1 };

    // check if we can remove first element
    if x == 0 { 
        let ord_yi: i32 = if levels[y] > levels[*i] { 2 } else { 1 };
        let diff_yi: i32 = (levels[y] - levels[*i]).abs();
        if diff_yi >= 1 && diff_yi <= 3 {
            *state = ord_yi; 
            *i += 1;
            return true;
        }
    } 

    if diff_xi >= 1 && diff_xi <= 3 && (ord_xi == *state || x == 0) {
        *i += 1;
        *state = ord_xi;
        return true;
    }

    false
}


fn violates(levels: &Vec<i32>, i: &usize, state: &mut i32) -> bool {
    let diff = (levels[*i] - levels[*i-1]).abs();
    if diff < 1 || diff > 3 {
        return true;
    }
    if levels[*i] > levels[*i-1] {
        if *state == 0 {
            *state = 1;
        } else if *state != 1{
            return true;
        }
    } 
    if levels[*i] < levels[*i-1] {
        if *state == 0 {
            *state = 2;
        } else if *state != 2{
            return true;
        }
    }
    false
}

fn is_safe(levels: Vec<i32>) -> bool {
    let len = levels.len();

    if len <= 2 {
        return true;
    }

    let mut state = 0; // null state, 1 = asc, 2 = desc
    let mut changed = false;
    let mut i = 1;
    while i < len {
        if violates(&levels, &mut i, &mut state) {
            if changed {
                return false;
            } else if !can_remove(&levels, &mut i, &mut state) {
                return false;
            }
            changed = true;
        } else {
            i += 1;
        }
    }
    true
}