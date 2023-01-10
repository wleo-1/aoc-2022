use aoc::HashSet;

pub fn run() {
    let input = std::fs::read_to_string("day-3/input.txt").unwrap();

    let mut answer = 0;

    for line in input.lines() {
        let mid = line.chars().count() / 2;
        let (first, second) = line.split_at(mid);

        let first = HashSet::from_iter(first.chars());
        let second = HashSet::from_iter(second.chars());

        let char = first.intersection(&second).next().unwrap();
        let zero = match char {
            'a'..='z' => 96,
            'A'..='Z' => 64 - 26,
            _ => panic!(),
        };

        answer += *char as u32 - zero;
    }

    println!("Part one: {answer}");
}
