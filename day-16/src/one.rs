use crate::*;

// process:
// graph with non-zero valves as nodes (plus AA) via floyd-warshall
// then just brute force it with recursion (and some optimizations)
pub fn run() {
    let time = std::time::Instant::now();
    let graph = create_graph("day-16/input.txt");
    let mut unvisited = graph
        .non_zero
        .iter()
        .map(|id| Some(*id))
        .collect::<Vec<_>>();

    let state = State {
        graph: &graph,
        id: *graph.node_ids.get("AA").unwrap(),
        pressure: 0,
        minutes: 30,
    };

    println!("Part one: {}", brute_force(&mut unvisited, 0, state));
    println!("Elapsed: {:?}", time.elapsed());
}
