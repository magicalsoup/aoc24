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
    
    part1::solve(&lines);
    part2::solve(&lines);
}
