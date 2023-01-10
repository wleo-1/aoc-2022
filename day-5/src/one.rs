use crate::*;

pub fn run() {
    let answer = calculate(|state, from, to, count| {
        for _ in 0..count {
            let item = state[from].pop().unwrap();
            state[to].push(item);
        }
    });

    println!("Part one: {answer}");
}
