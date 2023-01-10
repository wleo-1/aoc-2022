use crate::*;

pub fn run() {
    let grid = create_grid("day-15/input.txt");
    println!("Part one: {}", row_coverage(&grid, 2_000_000));
}

fn row_coverage(grid: &Grid, row: i64) -> usize {
    let mut ranges = Vec::new();

    for ((x, y), radius) in grid.sensors.iter() {
        let y_dif = row.abs_diff(*y);
        if let Some(offset) = radius.checked_sub(y_dif) {
            let offset = offset as i64;
            let (mut min, mut max) = (x - offset, x + offset);

            let (overlap, retain) = ranges
                .into_iter()
                .partition(|(vmin, vmax)| min <= *vmax && max >= *vmin);
            ranges = retain;

            for (omin, omax) in overlap {
                min = min.min(omin);
                max = max.max(omax);
            }

            ranges.push((min, max));
        }
    }

    let mut coverage = 0;

    for (min, max) in ranges {
        let beacons = grid
            .beacons
            .iter()
            .filter(|(x, y)| (min..=max).contains(x) && *y == row)
            .count();

        coverage += (max - min) as usize + 1 - beacons;
    }

    coverage
}
