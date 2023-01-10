use crate::*;

pub fn run() {
    let mut map = create_map("day-12/input.txt");
    println!("Part one: {}", pathfind(&mut map, |tile| tile.end));
}
