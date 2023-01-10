use aoc::HashSet;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

fn calculate(width: usize) -> usize {
    let input = std::fs::read_to_string("day-6/input.txt").unwrap();

    let mut offset = width;

    for window in input.chars().collect::<Vec<_>>().windows(width) {
        if HashSet::from_iter(window).len() == width {
            break;
        }

        offset += 1;
    }

    offset
}
