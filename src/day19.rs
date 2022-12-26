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

fn run_part1(bps: &Vec<BP>) -> i32 {
    bps.iter().for_each(|bp| println!("{}", bp));
    return 0;
}

//fn run_part2() -> i32 {
//    return 0;
//}
