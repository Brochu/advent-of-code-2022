type Cost = (u32, u32, u32);
type Resources = (u32, u32, u32, u32);
type Robots = (u32, u32, u32, u32);

struct BP {
    ore_gen_cost: Cost,
    clay_gen_cost: Cost,
    obsidian_gen_cost: Cost,
    geode_gen_cost: Cost,
}

fn parse_costs(line: &str) -> Cost {
    let mut costs = vec![0; 3];

    let (_, cost_str) = line.split_once("costs ").unwrap();
    let (amount, resource) = cost_str.split_once(" ").unwrap();

    if resource.contains("and") {
        let (resource, other) = resource.split_once(" and ").unwrap();
        match resource {
            "ore" => costs[0] = amount.parse::<u32>().unwrap(),
            "clay" => costs[1] = amount.parse::<u32>().unwrap(),
            "obsidian" => costs[2] = amount.parse::<u32>().unwrap(),
            _ => panic!("Could not parse resource type"),
        }

        //TODO: Find a way to make this less ugly, ark
        let (amount, resource) = other.split_once(" ").unwrap();
        match resource {
            "ore" => costs[0] = amount.parse::<u32>().unwrap(),
            "clay" => costs[1] = amount.parse::<u32>().unwrap(),
            "obsidian." => costs[2] = amount.parse::<u32>().unwrap(),
            _ => panic!("Could not parse resource type"),
        }
    }
    else {
        match resource {
            "ore" => costs[0] = amount.parse::<u32>().unwrap(),
            "clay" => costs[1] = amount.parse::<u32>().unwrap(),
            "obsidian" => costs[2] = amount.parse::<u32>().unwrap(),
            _ => panic!("Could not parse resource type"),
        }
    }

    return (costs[0], costs[1], costs[2]);
}

fn parse_bp(line: &str) -> BP {
    let (_, rest) = line.split_once(": ").unwrap();
    let (ore_str, rest) = rest.split_once(". ").unwrap();
    let (clay_str, rest) = rest.split_once(". ").unwrap();
    let (obsidian_str, geode_str) = rest.split_once(". ").unwrap();

    let costs = vec![
        parse_costs(ore_str),
        parse_costs(clay_str),
        parse_costs(obsidian_str),
        parse_costs(geode_str),
    ];

    return BP {
        ore_gen_cost: costs[0],
        clay_gen_cost: costs[1],
        obsidian_gen_cost: costs[2],
        geode_gen_cost: costs[3],
    };
}

pub fn main() {
    println!("[Day19] Solutions:");

    let blueprints = include_str!("../data/day19.example")
        .lines()
        .map(|line| parse_bp(line))
        .collect::<Vec<BP>>();

    println!("[Day19] Part 1 => {}", run_part1(&blueprints));
    //println!("[Day19] Part 2 => {}", run_part2());

    println!("[Day19] Complete -----------------------");
}

fn time_until(cost: &Cost, robots: &Robots, res: &Resources) -> u32 {
    let mut turns = 0;

    if robots.0 > 0 {
        let diff = cost.0 - res.0;
        println!("{} - {}", cost.0, res.0);
        let l_turns = diff / robots.0;
        println!("{} / {}", diff, robots.0);

        if l_turns > turns {  turns = l_turns }
    }

    if robots.1 > 0 {
        let diff = cost.1 - res.1;
        println!("{} - {}", cost.1, res.1);
        let l_turns = diff / robots.1;
        println!("{} / {}", diff, robots.1);

        if l_turns > turns {  turns = l_turns }
    }

    if robots.2 > 0 {
        let diff = cost.2 - res.2;
        println!("{} - {}", cost.2, res.2);
        let l_turns = diff / robots.2;
        println!("{} / {}", diff, robots.2);

        if l_turns > turns {  turns = l_turns }
    }

    return turns;
}

fn sim_turns(bp: &BP, idx: u32, robots: &Robots, res: &Resources, t: u32) -> (Robots, Resources) {
    let cost = match idx {
        0 => bp.ore_gen_cost,
        1 => bp.clay_gen_cost,
        2 => bp.obsidian_gen_cost,
        3 => bp.geode_gen_cost,
        _ => panic!("Invalid robot type"),
    };
    let new_res = (
        robots.0 * t + res.0 - cost.0,
        robots.1 * t + res.1 - cost.1,
        robots.2 * t + res.2 - cost.2,
        robots.3 * t + res.3,
    );

    let new_robots = match idx {
        0 => (robots.0+1, robots.1, robots.2, robots.3),
        1 => (robots.0, robots.1+1, robots.2, robots.3),
        2 => (robots.0, robots.1, robots.2+1, robots.3),
        3 => (robots.0, robots.1, robots.2, robots.3+1),
        _ => panic!("Could not simulate for this robot type"),
    };

    return (new_robots, new_res);
}

fn optimize_geodes(bp: &BP, robots: &Robots, res: &Resources, time: u32) -> u32 {
    let t = time_until(&bp.clay_gen_cost, robots, res) + 1;
    println!("Robots: {:?}, Resources: {:?}", robots, res);
    println!("Time: {}, t: {}", time, t);

    let mut results = Vec::new();
    if t <= time {
        let (rob, r) = sim_turns(&bp, 1, &robots, &res, t);
        results.push(optimize_geodes(bp, &rob, &r, time-t));
    }

    return 0;
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    const TIME: u32 = 24;
    let start_robots: Robots = (1, 0, 0, 0);
    let start_resources: Resources = (0, 0, 0, 0);

    return bps[0..1].iter()
        .enumerate()
        .map(|(idx, bp)| {
            (idx+1) as u32 * optimize_geodes(bp, &start_robots, &start_resources, TIME)
        })
        .sum();
}

//fn run_part2() -> usize {
//    return 0;
//}
