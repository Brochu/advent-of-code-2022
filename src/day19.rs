use std::{collections::HashMap, vec};

type Cost = (u32, u32, u32);
type Resources = (u32, u32, u32, u32);
type Robots = (u32, u32, u32, u32);
type Cache = HashMap<(Robots, Resources), u32>;

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

fn optimize_geodes(bp: &BP, robots: Robots, res: Resources, time: u32, cache: &mut Cache) -> u32 {
    // We already have the result cached
    if let Some(&result) = cache.get(&(robots, res)) { return result; }
    // We don't have anymore time, return how many geodes are opened
    if time == 0 { return res.3; }

    // Robots collect the resources
    let new_res = (
        res.0 + robots.0,
        res.1 + robots.1,
        res.2 + robots.2,
        res.3 + robots.3,
    );
    let mut max_res: u32 = 0;

    // Can we build an ore robot?
    if bp.ore_gen_cost.0 <= res.0 && bp.ore_gen_cost.1 <= res.1 && bp.ore_gen_cost.2 <= res.2 {
        let max = optimize_geodes(bp,
          (robots.0 + 1, robots.1, robots.2, robots.3),
          (new_res.0 - bp.ore_gen_cost.0, new_res.1 - bp.ore_gen_cost.1, new_res.2 - bp.ore_gen_cost.2, new_res.3),
          time-1,
          cache);

        if max > max_res { max_res = max }
    }

    // Can we build a clay robot
    if bp.clay_gen_cost.0 <= res.0 && bp.clay_gen_cost.1 <= res.1 && bp.clay_gen_cost.2 <= res.2 {
        let max = optimize_geodes(bp,
          (robots.0, robots.1 + 1, robots.2, robots.3),
          (new_res.0 - bp.clay_gen_cost.0, new_res.1 - bp.clay_gen_cost.1, new_res.2 - bp.clay_gen_cost.2, new_res.3),
          time-1,
          cache);

        if max > max_res { max_res = max }
    }

    // Can we build a obsidian robot
    if bp.obsidian_gen_cost.0 <= res.0 && bp.obsidian_gen_cost.1 <= res.1 && bp.obsidian_gen_cost.2 <= res.2 {
        let max = optimize_geodes(bp,
          (robots.0, robots.1, robots.2 + 1, robots.3),
          (new_res.0 - bp.obsidian_gen_cost.0, new_res.1 - bp.obsidian_gen_cost.1, new_res.2 - bp.obsidian_gen_cost.2, new_res.3),
          time-1,
          cache);

        if max > max_res { max_res = max }
    }

    // Can we build a geode robot
    if bp.geode_gen_cost.0 <= res.0 && bp.geode_gen_cost.1 <= res.1 && bp.geode_gen_cost.2 <= res.2 {
        let max = optimize_geodes(bp,
          (robots.0, robots.1, robots.2, robots.3 + 1),
          (new_res.0 - bp.geode_gen_cost.0, new_res.1 - bp.geode_gen_cost.1, new_res.2 - bp.geode_gen_cost.2, new_res.3),
          time-1,
          cache);

        if max > max_res { max_res = max }
    }

    let max = optimize_geodes(bp, robots, new_res, time-1, cache);
    if max > max_res { max_res = max }

    cache.insert((robots, res), max_res);
    return max_res;
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    const TIME: u32 = 24;
    let start_robots: Robots = (1, 0, 0, 0);
    let start_resources: Resources = (0, 0, 0, 0);

    //TODO: Solution works, but is too slow for real input
    return bps[0..1].iter()
        .enumerate()
        .map(|(idx, bp)| {
            let mut cache = Cache::new();
            (idx+1) as u32 * optimize_geodes(bp, start_robots, start_resources, TIME, &mut cache)
        })
        .sum();
}

//fn run_part2() -> usize {
//    return 0;
//}
