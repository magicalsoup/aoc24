
pub fn solve(grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut cnt = 0;
    for i in 0..rows {
        for j in 0..cols {
            horizontal_check(&grid, &i, &j, &mut cnt);
            vertical_check(&grid, &i, &j, &mut cnt);
            diagonal_check(&grid, &i, &j, &mut cnt);
        }
    }
    println!("{}", cnt);
}

fn horizontal_check(grid: &Vec<Vec<char>>, i: &usize, j: &usize, cnt: &mut i32) {
    let cols = grid[0].len();
    if *j + 3 >= cols {
        return;
    }

    if (grid[*i][*j] == 'X' && grid[*i][*j+1] == 'M' && grid[*i][*j+2] == 'A' && grid[*i][*j+3] == 'S') ||
       (grid[*i][*j] == 'S' && grid[*i][*j+1] == 'A' && grid[*i][*j+2] == 'M' && grid[*i][*j+3] == 'X') {
        *cnt += 1;
    }
}

fn vertical_check(grid: &Vec<Vec<char>>, i: &usize, j: &usize, cnt: &mut i32) {
    let rows = grid.len();
    if *i + 3 >= rows {
        return;
    }

    if (grid[*i][*j] == 'X' && grid[*i+1][*j] == 'M' && grid[*i+2][*j] == 'A' && grid[*i+3][*j] == 'S') ||
       (grid[*i][*j] == 'S' && grid[*i+1][*j] == 'A' && grid[*i+2][*j] == 'M' && grid[*i+3][*j] == 'X') {
        *cnt += 1;
    }
}

fn diagonal_check(grid: &Vec<Vec<char>>, i: &usize, j: &usize, cnt: &mut i32) {
    let rows = grid.len();
    let cols = grid[0].len();

    if *i + 3 < rows && *j + 3 < cols {
        if (grid[*i][*j] == 'X' && grid[*i+1][*j+1] == 'M' && grid[*i+2][*j+2] == 'A' && grid[*i+3][*j+3] == 'S') ||
           (grid[*i][*j] == 'S' && grid[*i+1][*j+1] == 'A' && grid[*i+2][*j+2] == 'M' && grid[*i+3][*j+3] == 'X') {
            *cnt += 1;
        }
    }

    if *i + 3 < rows && *j >= 3 {
        if (grid[*i][*j] == 'X' && grid[*i+1][*j-1] == 'M' && grid[*i+2][*j-2] == 'A' && grid[*i+3][*j-3] == 'S') ||
           (grid[*i][*j] == 'S' && grid[*i+1][*j-1] == 'A' && grid[*i+2][*j-2] == 'M' && grid[*i+3][*j-3] == 'X') {
            *cnt += 1;
        }
    }
}



