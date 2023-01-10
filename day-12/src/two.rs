use crate::*;

pub fn run() {
    let mut map = create_map("day-12/input.txt");
    println!(
        "Part two: {}",
        pathfind(&mut map, |tile| tile.elevation == 0)
    );
}
