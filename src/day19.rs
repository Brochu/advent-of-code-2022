use std::fmt::Display;

//enum Robot {
//    Ore,
//    Clay,
//    Obsidian,
//    Geode,
//}

struct BP {
    ore_robot_cost: i8,

    clay_robot_cost: i8,

    obs_robot_ore_cost: i8,
    obs_robot_clay_cost: i8,

    geo_robot_ore_cost: i8,
    geo_robot_obs_cost: i8,
}

impl Display for BP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blueprint:\n\tOre robot cost = {}\n\tClay robot cost = {}\n\tObsidian robot costs = {}, {}\n\tGeode robot costs = {}, {}", 
            self.ore_robot_cost,

            self.clay_robot_cost,

            self.obs_robot_ore_cost,
            self.obs_robot_clay_cost,

            self.geo_robot_ore_cost,
            self.geo_robot_obs_cost,
        )
    }
}

fn parse_blueprint(bp_str: &str) -> BP {
    let (_, bp_def) = bp_str.split_once(": ").unwrap();
    let mut robot_defs = bp_def.split(". ");

    let ore_def = robot_defs.next().unwrap();
    let (_, ore_def) = ore_def.split_once("costs ").unwrap();
    let (ore_def, _) = ore_def.split_once(" ").unwrap();

    let clay_def = robot_defs.next().unwrap();
    let (_, clay_def) = clay_def.split_once("costs ").unwrap();
    let (clay_def, _) = clay_def.split_once(" ").unwrap();

    let obsidian_def = robot_defs.next().unwrap();
    let (ore_cost, clay_cost) = obsidian_def.split_once(" and ").unwrap();
    let (_, ore_cost) = ore_cost.split_once("costs ").unwrap();
    let (ore_cost, _) = ore_cost.split_once(" ").unwrap();
    let (clay_cost, _) = clay_cost.split_once(" ").unwrap();

    let geode_def = robot_defs.next().unwrap();
    let (ore_cost_2, obsidian_cost) = geode_def.split_once(" and ").unwrap();
    let (_, ore_cost_2) = ore_cost_2.split_once("costs ").unwrap();
    let (ore_cost_2, _) = ore_cost_2.split_once(" ").unwrap();
    let (obsidian_cost, _) = obsidian_cost.split_once(" ").unwrap();

    return BP {
        ore_robot_cost: ore_def.parse().unwrap(),

        clay_robot_cost: clay_def.parse().unwrap(),

        obs_robot_ore_cost: ore_cost.parse().unwrap(),
        obs_robot_clay_cost: clay_cost.parse().unwrap(),

        geo_robot_ore_cost: ore_cost_2.parse().unwrap(),
        geo_robot_obs_cost: obsidian_cost.parse().unwrap(),
    };
}

struct State {
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    time_left: u32,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State ({} mins left):\n\tResources = ({:?})\n\tRobots = ({:?})\n", 
            self.time_left,
            vec![ self.ore_count, self.clay_count, self.obsidian_count, self.geode_count ],
            vec![ self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots ],
        )
    }
}

pub fn main() {
    println!("[Day19] Solutions:");

    let blueprints: Vec<_> = include_str!("../data/day19.example")
        .lines()
        .map(|line| parse_blueprint(line))
        .collect();

    println!("[Day19] Part 1 => {}", run_part1(&blueprints));
    //println!("[Day19] Part 2 => {}", run_part2());

    println!("[Day19] Complete -----------------------");
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    let mut _max_geodes: u32 = 0;

    for (idx, bp) in bps.iter().enumerate() {
        println!("[{}] {}", idx+1, bp);
        let mut _bp_max: u32 = 0;

        let mut stack = vec![ State {
            ore_count: 0, clay_count: 0, obsidian_count: 0, geode_count: 0,
            ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
            time_left: 24,
        }];

        while let Some(state) = stack.pop() {
            println!("{}", state);
        }

        if _bp_max > _max_geodes { _max_geodes = _bp_max }
    }

    return _max_geodes;
}

//fn run_part2() -> i32 {
//    return 0;
//}
