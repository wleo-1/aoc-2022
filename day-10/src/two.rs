pub fn run() {
    let input = std::fs::read_to_string("day-10/input.txt").unwrap();

    let mut output = Vec::new();
    let mut register = 1;
    let mut cycle = 0;

    for line in input.lines() {
        let mut line = aoc::Input::new(line);
        let instr = line.take(|char| char.is_ascii_alphabetic());

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

                do_cycle(&mut output, &mut cycle, register);
                do_cycle(&mut output, &mut cycle, register);

                register += sign * line.parse_int::<i32>();
            }
            "noop" => do_cycle(&mut output, &mut cycle, register),
            _ => panic!(),
        }
    }

    println!("Part two:");

    for row in output.chunks(40) {
        print!("\t");

        for pixel in row {
            let char = if *pixel { '#' } else { ' ' };

            print!("{char}");
        }

        print!("\n");
    }
}

fn do_cycle(output: &mut Vec<bool>, cycle: &mut i32, register: i32) {
    output.push(register.abs_diff(*cycle % 40) <= 1);
    *cycle += 1;
}
