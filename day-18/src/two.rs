use crate::*;
use std::collections::{HashSet, VecDeque};

pub fn run() {
    let droplet = create_droplet("day-18/input.txt");
    let mut min = [i32::MAX; 3];
    let mut max = [i32::MIN; 3];

    for &d in &droplet {
        min = zip(d, min, std::cmp::min);
        max = zip(d, max, std::cmp::max);
    }

    let min = min.map(|m| m - 1);
    let max = max.map(|m| m + 1);

    let shell = calc_shell(&droplet, min, max);
    let surface = calc_surface(&shell);

    let [x, y, z] = zip(min, max, |min, max| max - min + 1);
    let outside = 2 * (x * y + y * z + z * x);
    println!("Part two: {}", surface - outside as u32);
}

fn calc_shell(droplet: &HashSet<Coords>, min: Coords, max: Coords) -> HashSet<Coords> {
    let mut shell = HashSet::new();
    let mut queue = VecDeque::from([min]);

    while let Some(s) = queue.pop_front() {
        for n in NEIGHBORS {
            let s = zip(s, n, |s, n| s + n);
            let in_bounds = zip(s, min, |s, m| s >= m)
                .into_iter()
                .chain(zip(s, max, |s, m| s <= m))
                .all(std::convert::identity); // wow I can't believe I got to use this

            if in_bounds && !droplet.contains(&s) && !shell.contains(&s) {
                shell.insert(s);
                queue.push_back(s);
            }
        }
    }

    shell
}
