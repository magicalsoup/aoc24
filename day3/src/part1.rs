use regex::Regex;

pub fn solve(lines: &Vec<String>) {
    let mut concat = String::new();
    for line in lines {
        concat += line;
    }

    let s= concat.to_string();
    let mut sum: i32 = 0;
    let len = s.len();
    let mut i = 3;
    while i < len {
        let word = &s[i-3..i+1];
        //println!("word: {}", word);
        if word == "mul(" {
            // println!("matched");
            let mut params: [i32; 2] = [0;2];
            let mut j = 0;
            let mut bad = false;
            i += 1; 
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

pub fn regex_solve(lines: &Vec<String>) {
    let re = Regex::new(r"mul\([0-9]*,[0-9]*\)").unwrap();
    let mut concat = String::new();
    for line in lines {
        concat += line;
    }

    let s= concat.to_string();

    println!("{}", s);

    let matches: Vec<&str> = re.find_iter(&s).map(|mat| mat.as_str()).collect();
    let mut sum: i64 = 0;
    let mut cnt: i32 = 0;
    for m in matches {
        let f = m.replace("(", "").replace("m", "").replace("u", "").replace("l", "").replace(")", "");
        let parts: Vec<&str> = f.split(",").collect();
        let p1: i64 = parts[0].parse().unwrap();
        let p2: i64 = parts[1].parse().unwrap();
        let prod = p1 * p2;
        sum += prod;
        cnt += 1;
    }
    println!("cnt: {}, sum: {}", cnt, sum);
}