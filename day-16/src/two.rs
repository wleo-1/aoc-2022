use crate::*;
use aoc::rayon::prelude::*;

// since human and elephant are independent, just divy up nodes and add up totals at the end
pub fn run() {
    let time = std::time::Instant::now();
    let graph = create_graph("day-16/input.txt");
    let unvisited = graph.non_zero.clone();

    let animal = State {
        graph: &graph,
        id: *graph.node_ids.get("AA").unwrap(),
        pressure: 0,
        minutes: 26,
    };

    let max_bits = 2_u32.pow(unvisited.len() as u32);

    // rayon, heh...
    let best = (0..max_bits)
        .par_bridge()
        .filter(|bits| !bits & (max_bits - 1) >= *bits)
        .map(|mut bits| {
            let mut human = Vec::new();
            let mut elephant = Vec::new();

            for valve in unvisited.iter().cloned() {
                let animal = match bits & 1 {
                    1 => &mut human,
                    _ => &mut elephant,
                };

                animal.push(Some(valve));
                bits >>= 1;
            }

            let human = brute_force(&mut human, 0, animal.clone());
            let elephant = brute_force(&mut elephant, 0, animal.clone());

            human + elephant
        })
        .max()
        .unwrap();

    println!("Part two: {best}");
    println!("Elapsed: {:?}", time.elapsed());
}
