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

fn get_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    let rows = lines.len();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for i in 0..rows {
        let mut row: Vec<char> = Vec::new();
        for c in lines[i].chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
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
    let grid = get_grid(&lines);
    part1::solve(&grid);
    part2::solve(&grid);
}
