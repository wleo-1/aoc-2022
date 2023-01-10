mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

pub fn calculate(callback: impl Fn(&mut Vec<Vec<char>>, usize, usize, usize)) -> String {
    let input = std::fs::read_to_string("day-5/input.txt").unwrap();
    let mut lines = input.lines();

    let mut state = Vec::new();

    while let Some(line) = lines.next() {
        let mut chars = line.chars();

        match chars.next().unwrap() {
            '[' => {
                for idx in 0.. {
                    let char = chars.next().unwrap();

                    if char != ' ' {
                        let stack = match state.get_mut(idx) {
                            Some(stack) => stack,
                            None => {
                                for _ in 0..idx - state.len() + 1 {
                                    state.push(Vec::new());
                                }

                                state.last_mut().unwrap()
                            }
                        };

                        stack.insert(0, char);
                    }

                    if chars.by_ref().take(3).count() < 3 {
                        break;
                    }
                }
            }
            ' ' => break,
            _ => panic!(),
        }
    }

    lines.next();

    for line in lines {
        let mut chars = aoc::Input::new(line);

        chars.skip(5); // "move "
        let count = chars.parse_int::<usize>();
        chars.skip(6); // " from "
        let from = chars.parse_int::<usize>() - 1;
        chars.skip(4); // " to "
        let to = chars.parse_int::<usize>() - 1;

        callback(&mut state, from, to, count);
    }

    let mut answer = String::new();

    for stack in state {
        answer.push(*stack.last().unwrap());
    }

    answer
}
