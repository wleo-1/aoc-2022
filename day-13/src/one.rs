use crate::*;

pub fn run() {
    let input = std::fs::read_to_string("day-13/input.txt").unwrap();
    let mut input = input.lines().peekable();

    let mut idx = 0;
    let mut sum = 0;

    while let Some(peek) = input.peek() {
        if peek.is_empty() {
            input.next();
            continue;
        }

        idx += 1;

        if let Ordering::Less = calculate(input.next().unwrap(), input.next().unwrap()) {
            sum += idx;
        }
    }

    println!("Part one: {sum}");
}
