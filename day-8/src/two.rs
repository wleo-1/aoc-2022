use crate::*;

pub fn run() {
    let mut grid = create_grid("day-8/input.txt");
    println!("Part two: {}", optimal_tree(&mut grid));
}

fn check_line(grid: &mut Grid, height: u32, iter: impl Iterator<Item = (usize, usize)>) -> u32 {
    let mut count = 0;

    for (row, col) in iter {
        count += 1;

        let tree = &mut grid.trees[row][col];

        if tree.height >= height {
            break;
        }
    }

    count
}

fn calculate_value(grid: &mut Grid, (row, col): (usize, usize)) -> u32 {
    let rows = grid.trees.len();
    let cols = grid.trees[0].len();

    let height = grid.trees[row][col].height;

    let down = check_line(grid, height, (col + 1..cols).map(|col| (row, col)));
    let up = check_line(grid, height, (0..col).rev().map(|col| (row, col)));

    let right = check_line(grid, height, (row + 1..rows).map(|row| (row, col)));
    let left = check_line(grid, height, (0..row).rev().map(|row| (row, col)));

    up * down * left * right
}

fn optimal_tree(grid: &mut Grid) -> u32 {
    let rows = grid.trees.len();
    let cols = grid.trees[0].len();

    let mut best = 0;

    for row in 0..rows {
        for col in 0..cols {
            best = calculate_value(grid, (row, col)).max(best);
        }
    }

    best
}
