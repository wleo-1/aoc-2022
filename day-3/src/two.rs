use aoc::HashSet;

pub fn run() {
    let input = std::fs::read_to_string("day-3/input.txt").unwrap();

    let mut answer = 0;

    for lines in input.lines().collect::<Vec<_>>().chunks(3) {
        let [a, b, c] = lines else {
            panic!();
        };

        let a = HashSet::from_iter(a.chars());
        let b = HashSet::from_iter(b.chars());
        let c = HashSet::from_iter(c.chars());

        let intersect_ab = a.intersection(&b).cloned().collect::<HashSet<_>>();
        let mut intersect_abc = intersect_ab.intersection(&c).cloned();
        let char = intersect_abc.next().unwrap();

        // all this is good
        let zero = match char {
            'a'..='z' => 96,
            'A'..='Z' => 64 - 26,
            _ => panic!(),
        };

        answer += char as u32 - zero;
    }

    println!("Part two: {answer}");
}
