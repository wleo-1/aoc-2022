pub fn run() {
    let input = std::fs::read_to_string("day-4/input.txt").unwrap();

    let mut count = 0;

    for line in input.lines() {
        let mut chars = line.chars();

        let mut take = || {
            chars
                .by_ref()
                .take_while(|char| char.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        };

        let a_min = take();
        let a_max = take();
        let b_min = take();
        let b_max = take();

        if !(a_min > b_max || a_max < b_min) {
            count += 1;
        }
    }

    println!("Part two: {count}");
}
