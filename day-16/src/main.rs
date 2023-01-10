use aoc::Itertools;
use std::{collections::HashMap, iter::repeat};

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

const VALVE_COUNT: usize = 54;

struct Graph {
    node_ids: HashMap<String, usize>,
    flow_rates: Vec<u32>,      // get ids from here
    dist_table: Vec<Vec<u32>>, // 0 is infinity
    non_zero: Vec<usize>,
    shortest: u32,
}

#[derive(Clone)]
struct State<'a> {
    graph: &'a Graph,
    id: usize,
    pressure: u32,
    minutes: u32,
}

fn get_or_create_id(graph: &mut Graph, str: String) -> usize {
    if let Some(id) = graph.node_ids.get(&str) {
        return *id;
    }

    let id = graph.flow_rates.len();
    graph.flow_rates.push(0);
    graph.node_ids.insert(str, id);

    id
}

fn create_graph(path: &str) -> Graph {
    let input = std::fs::read_to_string(path).unwrap();
    let row = repeat(u32::MAX).take(VALVE_COUNT).collect();
    let mut graph = Graph {
        node_ids: HashMap::new(),
        flow_rates: Vec::new(),
        dist_table: repeat(row).take(VALVE_COUNT).collect(),
        non_zero: Vec::new(),
        shortest: u32::MAX,
    };

    for line in input.lines() {
        let mut chars = line.chars().peekable();

        chars.by_ref().take(6).for_each(drop); // "Valve "

        let name = chars.by_ref().take(2).collect();
        let id = get_or_create_id(&mut graph, name);
        graph.dist_table[id][id] = 0;

        chars.by_ref().take(15).for_each(drop); // " has flow rate="

        let flow_rate = chars
            .by_ref()
            .peeking_take_while(|char| char.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        graph.flow_rates[id] = flow_rate;

        if flow_rate != 0 {
            graph.non_zero.push(id);
        }

        while let Some(peek) = chars.peek() {
            match peek {
                'A'..='Z' => {
                    let to_name = chars.by_ref().take(2).collect();
                    let to_id = get_or_create_id(&mut graph, to_name);

                    graph.dist_table[id][to_id] = 1;
                    graph.dist_table[to_id][id] = 1;
                }
                _ => {
                    chars.next();
                }
            }
        }
    }

    // floyd-warshall
    for k in 0..VALVE_COUNT {
        for i in 0..VALVE_COUNT {
            for j in 0..VALVE_COUNT {
                let v_kj = graph.dist_table[k][j];
                let v_i = &mut graph.dist_table[i];
                v_i[j] = v_i[j].min(v_i[k].saturating_add(v_kj));
            }
        }
    }

    graph
        .non_zero
        .sort_by(|a, b| graph.flow_rates[*a].cmp(&graph.flow_rates[*b]));

    for i in 0..graph.non_zero.len() - 1 {
        for b in &graph.non_zero[i..] {
            let dist = graph.dist_table[graph.non_zero[i]][*b];
            graph.shortest = graph.shortest.min(dist);
        }
    }

    graph
}

fn brute_force(unvisited: &mut [Option<usize>], mut best: u32, state: State) -> u32 {
    best = state.pressure.max(best);

    let potential = (0..state.minutes)
        .rev()
        .step_by(state.graph.shortest as usize + 1)
        .zip(unvisited.iter().filter_map(|id| *id))
        .fold(state.pressure, |pressure, (mins, id)| {
            pressure + state.graph.flow_rates[id] * mins
        });

    if potential < best {
        return 0;
    }

    for idx in 0..unvisited.len() {
        let Some(id) = unvisited[idx].take() else {
            continue;
        };

        let dist = state.graph.dist_table[state.id][id];

        if let Some(minutes) = state.minutes.checked_sub(dist + 1) {
            best = brute_force(
                unvisited,
                best,
                State {
                    graph: state.graph,
                    id,
                    pressure: state.pressure + state.graph.flow_rates[id] * minutes,
                    minutes,
                },
            )
            .max(best);
        }

        unvisited[idx] = Some(id);
    }

    best
}
