use std::cmp::Ordering;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

struct Node {
    val: i64,
    prev: usize,
    next: usize,
}

fn decrypt(path: &str, key: i64, count: usize) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut nodes = Vec::<Node>::new();
    let mut order = Vec::new();
    let mut zero = 0;

    for (idx, line) in input.lines().enumerate() {
        let mut node = Node {
            val: line.parse::<i64>().unwrap() * key,
            prev: 0,
            next: 0,
        };

        if node.val == 0 {
            zero = idx;
        }

        if idx > 0 {
            node.prev = idx - 1;
            nodes[node.prev].next = idx;
        }

        nodes.push(node);
        order.push(idx);
    }

    let last = nodes.len() - 1;
    nodes[0].prev = last;
    nodes[last].next = 0;

    for _ in 0..count {
        all_nodes(&mut nodes, &order);
    }

    let mut coords = std::iter::successors(Some(zero), |idx| Some(nodes[*idx].next)).step_by(1000);

    coords.next().map(|idx| nodes[idx].val); // zero
    coords.take(3).map(|idx| nodes[idx].val).sum()
}

fn do_node(nodes: &mut [Node], idx: usize) {
    let Node { val, prev, next } = nodes[idx];
    nodes[prev].next = next;
    nodes[next].prev = prev;

    let (prev, next) = 'a: {
        let fold = match val.cmp(&0) {
            Ordering::Equal => break 'a (prev, next),
            Ordering::Greater => |nodes: &[Node], _, next: usize| (next, nodes[next].next),
            Ordering::Less => |nodes: &[Node], prev: usize, _| (nodes[prev].prev, prev),
        };

        (0..val.abs() % (nodes.len() as i64 - 1)).fold((prev, next), |(prev, next), _| fold(&nodes, prev, next))
    };

    nodes[idx].prev = prev;
    nodes[idx].next = next;
    nodes[prev].next = idx;
    nodes[next].prev = idx;
}

fn all_nodes(nodes: &mut [Node], order: &[usize]) {
    for idx in order {
        do_node(nodes, *idx);
    }
}
