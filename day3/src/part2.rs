pub fn solve(lines: &Vec<String>) {
    let mut concat = String::new();
    for line in lines {
        concat += line;
    }

    let s= concat.to_string();
    let mut sum: i32 = 0;
    let len = s.len();
    let mut i = 6;
    let mut enabled = true;
    while i < len {
        let chunk = &s[i-6..i+1];
        let mul_instr = &chunk[0..4];
        let do_instr = &chunk[0..4]; 
        if chunk == "don't()" {
            enabled = false;
        } else if do_instr == "do()" {
            enabled = true;
        } else if mul_instr == "mul(" && enabled {
            let mut params: [i32; 2] = [0;2];
            let mut j = 0;
            let mut bad = false;
            i -= 2; 
            while i < len {
                let substr = &s[i..i+1];
                let c = substr.chars().next().unwrap();
                if c == ')' {
                    break;
                } else if c == ',' {
                    j += 1;
                    if j >= 2 {
                        bad = true;
                        break;
                    }
                } else if c.is_ascii_digit() {
                    let d: i32 = c.to_string().parse().unwrap();
                    params[j] = params[j] * 10 + d;
                } else {
                    bad = true;
                    break;
                }
                i += 1; 
            }

            if i >= len && &s[len-1..len] != ")" {
                bad = true;
            }
            if !bad {
                sum += params[0] * params[1];
            }
        }
        i += 1;
    }
    println!("{}", sum);
}
