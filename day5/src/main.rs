use std::fs::read_to_string;

pub mod part1;
pub mod part2;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

const TEST_FLAG: bool = false;

fn main() {
    
    let filename = 
        if TEST_FLAG { 
            "test.txt"
        } else { 
            "input.txt"
        };

    let lines = read_lines(filename);

    let mut init = true;
    let mut init_lines: Vec<String> = Vec::new();
    let mut query_lines: Vec<String> = Vec::new();

    for line in lines {
        if line == "" {
            init = false;
        } else if init {
            init_lines.push(line);
        } else {
            query_lines.push(line);
        }
    }

    part1::solve(&init_lines, &query_lines);
    part2::solve(&init_lines, &query_lines);
}
