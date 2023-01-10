use crate::*;

pub fn run() {
    let answer = calculate(|state, from, to, count| {
        // borrowck, oh my
        let (from, to) = if from < to {
            let (from_slice, to_slice) = state.split_at_mut(to);
            (&mut from_slice[from], &mut to_slice[0])
        } else {
            let (to_slice, from_slice) = state.split_at_mut(from);
            (&mut from_slice[0], &mut to_slice[to])
        };

        let idx = from.len() - count;
        from.truncate(idx);
        to.extend(&from[idx..]);
    });

    println!("Part two: {answer}");
}
