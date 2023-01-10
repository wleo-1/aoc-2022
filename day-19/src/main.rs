mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

struct Blueprint {
    id: u32,
    costs: [[u32; 3]; 4],
    maxmimums: [u32; 4],
}

fn create_blueprint(line: &str) -> Blueprint {
    let mut extr = aoc::Extract::compile(concat!(
        "Blueprint {id:n}:",
        " Each ore robot costs {ore:n} ore.",
        " Each clay robot costs {clay:n} ore.",
        " Each obsidian robot costs {obby_ore:n} ore and {obby_clay:n} clay.",
        " Each geode robot costs {geo_ore:n} ore and {geo_obby:n} obsidian.",
    ));
    extr.parse_str(line);

    macro_rules! get {
        ($key:literal) => {
            extr.get::<u32>($key)
        };
    }

    let mut bp = Blueprint {
        id: get!("id"),
        costs: [
            [get!("ore"), 0, 0],
            [get!("clay"), 0, 0],
            [get!("obby_ore"), get!("obby_clay"), 0],
            [get!("geo_ore"), 0, get!("geo_obby")],
        ],
        maxmimums: [u32::MAX; 4],
    };

    bp.maxmimums[1] = bp.costs[2][1];
    bp.maxmimums[2] = bp.costs[3][2];

    bp
}

// use bitmasks for resources and robots; FOUR that acts like [ONE; 4]
fn do_robots(
    mut best: u32,
    bp: &Blueprint,
    mins: u32,
    resources: [u32; 4],
    robots: [u32; 4],
    skip: u8,
) -> u32 {
    if mins == 1 {
        return best.max(resources[3] + robots[3]);
    }

    // ignore branches that cannot possibly be higher than the current best
    let init = (resources[2], robots[2], resources[3] + mins * robots[3]);
    let (_, _, potential) = (0..mins).rev().fold(init, |(obby, robby, geodes), mins| {
        if obby >= bp.costs[3][2] {
            (obby + robby - bp.costs[3][2], robby, geodes + mins)
        } else {
            (obby + robby, robby + 1, geodes)
        }
    });

    if potential < best {
        return 0;
    }

    let start = if mins == 2 { 3 } else { 0 };
    let available_robots = (start..4)
        .rev()
        .filter(|&i| skip & (1 << i) == 0)
        .filter(|&i| robots[i] < bp.maxmimums[i])
        .filter(|&i| (0..3).all(|r| resources[r] >= bp.costs[i][r]));

    let mut resources = resources;
    for i in 0..4 {
        resources[i] += robots[i];
    }

    let mut skip = 0;

    for i in available_robots {
        skip ^= 1 << i;

        let mut new_resources = resources;
        for r in 0..3 {
            new_resources[r] -= bp.costs[i][r];
        }

        let mut new_robots = robots;
        new_robots[i] += 1;

        best = do_robots(best, bp, mins - 1, new_resources, new_robots, 0).max(best);
    }

    skip &= !0b1000;

    // ... or do nothing
    // also skip branches if they were available in the previous iteration but not chosen
    do_robots(best, bp, mins - 1, resources, robots, skip).max(best)
}
