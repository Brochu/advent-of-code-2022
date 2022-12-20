use std::collections::HashMap;

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

    return BP {
        ore_gen_cost: parse_costs(ore_str),
        clay_gen_cost: parse_costs(clay_str),
        obsidian_gen_cost: parse_costs(obsidian_str),
        geode_gen_cost: parse_costs(geode_str),
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
    if let Some(&result) = cache.get(&(robots, res)) {
        //println!("Cached result: {:?}, {:?}, {}", robots, res,  time);
        return result;
    }

    if time == 0 {
        //println!("Found new result: {:?}, {:?}, {}", robots, res,  time);
        return res.3;
    }

    // Find possibilities for builds

    let max_res = optimize_geodes(bp, robots, res, time-1, cache);
    cache.insert((robots, res), max_res);
    return max_res;
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    const TIME: u32 = 24;
    let start_robots: Robots = (1, 0, 0, 0);
    let start_resources: Resources = (0, 0, 0, 0);

    return bps.iter()
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
