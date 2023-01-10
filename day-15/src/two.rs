use crate::*;

const MAX: i64 = 4000000;

pub fn run() {
    let grid = create_grid("day-15/input.txt");
    let (x, y) = find_beacon(&grid);
    println!("Part two: {}", x * MAX + y);
}

fn check_not_covered(grid: &Grid, (x, y): (i64, i64)) -> bool {
    grid.sensors
        .iter()
        .all(|((sx, sy), radius)| x.abs_diff(*sx) + y.abs_diff(*sy) > *radius)
}

fn check_pair(grid: &Grid, a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    let manhattan = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
    let ar = grid.sensors.get(&a).unwrap();
    let br = grid.sensors.get(&b).unwrap();

    if manhattan != ar + br + 2 {
        return None;
    }

    let ((a, ar), b) = if ar < br { ((a, ar), b) } else { ((b, br), a) };
    let ar = *ar as i64;
    let x_dir = (b.0 - a.0).signum();
    let y_dir = (b.1 - a.1).signum();

    let to = (a.0, a.1 + y_dir * (ar + 1));
    let mut curr = (a.0 + x_dir * (ar + 1), a.1);

    loop {
        let in_range = (0..=MAX).contains(&curr.0) && (0..=MAX).contains(&curr.1);
        if in_range && !grid.beacons.contains(&curr) && check_not_covered(grid, curr) {
            return Some(curr);
        }

        if curr == to {
            break;
        }

        curr.0 -= x_dir;
        curr.1 += y_dir;
    }

    None
}

fn find_beacon(grid: &Grid) -> (i64, i64) {
    for a in grid.sensors.keys() {
        for b in grid.sensors.keys() {
            if a != b {
                if let Some(beacon) = check_pair(grid, *a, *b) {
                    return beacon;
                }
            }
        }
    }

    panic!();
}
