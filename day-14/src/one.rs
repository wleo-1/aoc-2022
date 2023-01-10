use crate::*;

pub fn run() {
    let mut cave = create_cave("day-14/input.txt");

    while !do_sand(&mut cave) {}

    println!("Part one: {}", cave.count);
}

// returns abyss?
fn do_sand(cave: &mut Cave) -> bool {
    let mut pos = (500, 0);

    loop {
        assert!(!cave.occupied.contains(&pos));

        let mut new_iter = DIRS
            .into_iter()
            .map(|(x_dif, y_dif)| (pos.0 + x_dif, pos.1 + y_dif))
            .filter(|pos| !cave.occupied.contains(pos));

        if let Some(new) = new_iter.next() {
            if pos.1 >= cave.abyss {
                return true;
            }
            pos = new;
        } else {
            cave.occupied.insert(pos);
            cave.count += 1;
            return false;
        }
    }
}
