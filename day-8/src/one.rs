use crate::*;

pub fn run() {
    let mut grid = create_grid("day-8/input.txt");

    check_all(&mut grid);

    println!("Part one: {}", grid.visible);
}

fn check_line(grid: &mut Grid, iter: impl Iterator<Item = (usize, usize)>) {
    let mut highest = 0;

    for (row, col) in iter {
        let tree = &mut grid.trees[row][col];

        if tree.height > highest {
            highest = tree.height;

            if !tree.seen {
                tree.seen = true;
                grid.visible += 1;
            }
        }
    }
}

fn check_all(grid: &mut Grid) {
    let rows = grid.trees.len();
    let cols = grid.trees[0].len();

    for row in 0..rows {
        check_line(grid, (0..cols).map(|col| (row, col)));
        check_line(grid, (0..cols).rev().map(|col| (row, col)));
    }

    for col in 0..cols {
        check_line(grid, (0..rows).map(|row| (row, col)));
        check_line(grid, (0..rows).rev().map(|row| (row, col)));
    }
}
