use std::fmt::Display;

#[derive(Debug)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct BP {
    ore_robot_cost: u8,

    clay_robot_cost: u8,

    obs_robot_ore_cost: u8,
    obs_robot_clay_cost: u8,

    geo_robot_ore_cost: u8,
    geo_robot_obs_cost: u8,
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
    ore_count: u8,
    clay_count: u8,
    obsidian_count: u8,
    geode_count: u8,

    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,

    time: u8,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State (Time: {}):\n\tResources = ({:?})\n\tRobots = ({:?})\n", 
            self.time,
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

fn robot_possible(state: &State, robot: Robot) -> bool {
    match robot {
        Robot::Ore => true,
        Robot::Clay => true,
        Robot::Obsidian => state.ore_robots > 0 && state.clay_robots > 0,
        Robot::Geode => state.ore_robots > 0 && state.obsidian_robots > 0,
    }
}

fn turns_to_build(bp: &BP, state: &State, robot: Robot) -> u8 {
    match robot {
        Robot::Ore => (bp.ore_robot_cost - state.ore_count) / state.ore_robots,
        Robot::Clay => (bp.clay_robot_cost - state.ore_count) / state.ore_robots,
        Robot::Obsidian => {
            ((bp.obs_robot_ore_cost - state.ore_count) / state.ore_robots).max(
                (bp.obs_robot_clay_cost - state.clay_count) / state.clay_robots
            )
        },
        Robot::Geode => {
            ((bp.geo_robot_ore_cost - state.ore_count) / state.ore_robots).max(
                (bp.geo_robot_obs_cost - state.obsidian_count) / state.obsidian_robots
            )
        },
    }
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    let mut quality_sum: u32 = 0;

    for (idx, bp) in bps[0..1].iter().enumerate() {
        println!("[{}] {}", idx+1, bp);
        let mut bp_max: u32 = 0;

        let mut stack = vec![ State {
            ore_count: 0, clay_count: 0, obsidian_count: 0, geode_count: 0,
            ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
            time: 0,
        }];

        while let Some(state) = stack.pop() {
            println!("{}", state);

            if state.time == 24 {
                // This branch is over
                if state.geode_count as u32 > bp_max { bp_max = state.geode_count as u32 }
                continue;
            }

            if robot_possible(&state, Robot::Geode) {
                let t = turns_to_build(&bp, &state, Robot::Geode) + 1;
                if state.time + t <= 24 {
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t) - bp.geo_robot_ore_cost,
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t) - bp.geo_robot_obs_cost,
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots + 1,

                        time: state.time + t,
                    });
                }
                else {
                    let t = 24 - state.time;
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t),
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
            }

            if robot_possible(&state, Robot::Obsidian) {
                let t = turns_to_build(&bp, &state, Robot::Obsidian) + 1;
                if state.time + t <= 24 {
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t) - bp.obs_robot_ore_cost,
                        clay_count: state.clay_count + (state.clay_robots * t) - bp.obs_robot_clay_cost,
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots + 1,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
                else {
                    let t = 24 - state.time;
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t),
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
            }

            if robot_possible(&state, Robot::Clay) {
                let t = turns_to_build(&bp, &state, Robot::Clay) + 1;
                if state.time + t <= 24 {
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t) - bp.clay_robot_cost,
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots + 1,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
                else {
                    let t = 24 - state.time;
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t),
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
            }

            if robot_possible(&state, Robot::Ore) {
                let t = turns_to_build(&bp, &state, Robot::Ore) + 1;
                if state.time + t <= 24 {
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t) - bp.ore_robot_cost,
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots + 1,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
                else {
                    let t = 24 - state.time;
                    stack.push(State {
                        ore_count: state.ore_count + (state.ore_robots * t),
                        clay_count: state.clay_count + (state.clay_robots * t),
                        obsidian_count: state.obsidian_count + (state.obsidian_robots * t),
                        geode_count: state.geode_count + (state.geode_robots * t),

                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,

                        time: state.time + t,
                    });
                }
            }
        }

        quality_sum += bp_max * (idx+1) as u32
    }

    return quality_sum;
}

//fn run_part2() -> i32 {
//    return 0;
//}
