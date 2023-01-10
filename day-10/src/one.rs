pub fn run() {
    let input = std::fs::read_to_string("day-10/input.txt").unwrap();

    let mut cycle = 1;
    let mut register = 1;
    let mut sum = 0;
    let mut checkpoints = [20, 60, 100, 140, 180, 220].into_iter().peekable();

    for line in input.lines() {
        let mut line = aoc::Input::new(line);
        let instr = line.take(|char| char.is_ascii_alphabetic());
        let old = register;

        match instr.as_str() {
            "addx" => {
                line.skip(1);

                let sign = match line.peek().unwrap() {
                    '-' => {
                        line.next();
                        -1
                    }
                    _ => 1,
                };

                register += sign * line.parse_int::<i32>();
                cycle += 2;
            }
            "noop" => cycle += 1,
            _ => panic!(),
        }

        let Some(check) = checkpoints.peek() else {
            break;
        };

        if cycle > *check {
            sum += *check * old;
            checkpoints.next();
        }
    }

    println!("Part one: {sum}");
}
