use crate::*;

pub fn run() {
    let mut cave = create_cave("day-14/input.txt");

    while !cave.occupied.contains(&(500, 0)) {
        do_sand(&mut cave);
    }

    println!("Part two: {}", cave.count);
}

// returns abyss?
fn do_sand(cave: &mut Cave) {
    let mut pos = (500, 0);

    loop {
        assert!(!cave.occupied.contains(&pos));

        let mut new_iter = DIRS
            .into_iter()
            .map(|(x_dif, y_dif)| (pos.0 + x_dif, pos.1 + y_dif))
            .filter(|pos| !cave.occupied.contains(pos))
            .filter(|pos| pos.1 < cave.abyss + 2);

        if let Some(new) = new_iter.next() {
            pos = new;
        } else {
            cave.occupied.insert(pos);
            cave.count += 1;
            return;
        }
    }
}
