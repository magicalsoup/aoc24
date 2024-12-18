pub fn solve(grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut cnt = 0;
    for i in 0..rows {
        for j in 0..cols {
            xmas_check(&grid, &i, &j, &mut cnt);
        }
    }
    println!("{}", cnt);
}

fn xmas_check(grid: &Vec<Vec<char>>, i: &usize, j: &usize, cnt: &mut i32) {
    let mut right_xmas = false;
    let mut left_xmas = false;

    let rows = grid.len();
    let cols = grid[0].len();

    if *i + 2 < rows && *j + 2 < cols {

        if (grid[*i][*j] == 'M' && grid[*i+1][*j+1] == 'A' && grid[*i+2][*j+2] == 'S') ||
           (grid[*i][*j] == 'S' && grid[*i+1][*j+1] == 'A' && grid[*i+2][*j+2] == 'M') {
            right_xmas = true;
        }
        if (grid[*i][*j+2] == 'M' && grid[*i+1][*j+1] == 'A' && grid[*i+2][*j] == 'S') ||
            (grid[*i][*j+2] == 'S' && grid[*i+1][*j+1] == 'A' && grid[*i+2][*j] == 'M') {
            left_xmas = true;
        }
    }

    if right_xmas && left_xmas {
        *cnt += 1;
    }
}