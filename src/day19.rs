use std::fmt::Display;
use std::time::Instant;

struct BP {
    or_c: u32,
    cl_c: u32,
    ob_or_c: u32, ob_cl_c: u32,
    ge_or_c: u32, ge_ob_c: u32,

    or_m: u32,
    cl_m: u32,
    ob_m: u32,
}

impl Display for BP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blueprint:\n\tOre robot cost = {}\n\tClay robot cost = {}\n\tObsidian robot costs = {}, {}\n\tGeode robot costs = {}, {}\n\tMax = {}, {}, {}", 
            self.or_c,
            self.cl_c,
            self.ob_or_c, self.ob_cl_c,
            self.ge_or_c, self.ge_ob_c,
            self.or_m, self.cl_m, self.ob_m,
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

    let or_c = ore_def.parse().unwrap();
    let cl_c = clay_def.parse().unwrap();
    let ob_or_c = ore_cost.parse().unwrap();
    let ob_cl_c = clay_cost.parse().unwrap();
    let ge_or_c = ore_cost_2.parse().unwrap();
    let ge_ob_c = obsidian_cost.parse().unwrap();

    return BP {
        or_c ,
        cl_c ,
        ob_or_c , ob_cl_c ,
        ge_or_c , ge_ob_c ,

        or_m: or_c.max(cl_c.max(ob_or_c.max(ge_or_c))),
        cl_m: ob_cl_c,
        ob_m: ge_ob_c,
    };
}

struct State {
    or: u32, cl: u32, ob: u32, ge: u32,
    or_r: u32, cl_r: u32, ob_r: u32, ge_r: u32,
    time: u32,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State (Time: {}):\n\tResources = ({:?})\n\tRobots = ({:?})\n", 
            self.time,
            vec![ self.or, self.cl, self.ob, self.ge ],
            vec![ self.or_r, self.cl_r, self.ob_r, self.ge_r ],
        )
    }
}

pub fn main() {
    println!("[Day19] Solutions:");

    let blueprints: Vec<_> = include_str!("../data/day19.input")
        .lines()
        .map(|line| parse_blueprint(line))
        .collect();

    println!("[Day19] Part 1 => {}", run_part1(&blueprints));
    //println!("[Day19] Part 2 => {}", run_part2());

    println!("[Day19] Complete -----------------------");
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    let mut quality_sum: u32 = 0;
    let now = Instant::now();

    for (idx, bp) in bps[..].iter().enumerate() {
        println!("[{}] {}", idx+1, bp);
        let mut bp_max: u32 = 0;

        let mut stack = vec![ State {
            or: 0, cl: 0, ob: 0, ge: 0,
            or_r: 1, cl_r: 0, ob_r: 0, ge_r: 0,
            time: 0,
        }];

        while let Some(state) = stack.pop() {
            //println!("{}", state);

            if state.time == 24 {
                // This branch is over
                if state.ge > bp_max { bp_max = state.ge }
                continue;
            }

            if state.or >= bp.ge_or_c  && state.ob >= bp.ge_ob_c {
                stack.push(State {
                    or: state.or + state.or_r - bp.ge_or_c,
                    cl: state.cl + state.cl_r,
                    ob: state.ob + state.ob_r - bp.ge_ob_c,
                    ge: state.ge + state.ge_r,
                    or_r: state.or_r, cl_r: state.cl_r, ob_r: state.ob_r, ge_r: state.ge_r + 1,
                    time: state.time + 1,
                });
            }

            if state.ob_r < bp.ob_m && state.or >= bp.ob_or_c && state.cl >= bp.ob_cl_c {
                stack.push(State {
                    or: state.or + state.or_r - bp.ob_or_c,
                    cl: state.cl + state.cl_r - bp.ob_cl_c,
                    ob: state.ob + state.ob_r,
                    ge: state.ge + state.ge_r,
                    or_r: state.or_r, cl_r: state.cl_r, ob_r: state.ob_r + 1, ge_r: state.ge_r,
                    time: state.time + 1,
                });
            }

            if state.cl_r < bp.cl_m && state.or >= bp.cl_c {
                stack.push(State {
                    or: state.or + state.or_r - bp.cl_c,
                    cl: state.cl + state.cl_r,
                    ob: state.ob + state.ob_r,
                    ge: state.ge + state.ge_r,
                    or_r: state.or_r, cl_r: state.cl_r + 1, ob_r: state.ob_r, ge_r: state.ge_r,
                    time: state.time + 1,
                });
            }

            if state.or_r < bp.or_m && state.or >= bp.or_c {
                stack.push(State {
                    or: state.or + state.or_r - bp.or_c,
                    cl: state.cl + state.cl_r,
                    ob: state.ob + state.ob_r,
                    ge: state.ge + state.ge_r,
                    or_r: state.or_r + 1, cl_r: state.cl_r, ob_r: state.ob_r, ge_r: state.ge_r,
                    time: state.time + 1,
                });
            }

            stack.push(State {
                or: state.or + state.or_r,
                cl: state.cl + state.cl_r,
                ob: state.ob + state.ob_r,
                ge: state.ge + state.ge_r,
                or_r: state.or_r, cl_r: state.cl_r, ob_r: state.ob_r, ge_r: state.ge_r,
                time: state.time + 1,
            });
        }

        quality_sum += bp_max * (idx+1) as u32
    }

    println!("Time spent for this part: {}", now.elapsed().as_secs());
    return quality_sum;
}

//fn run_part2() -> i32 {
//    return 0;
//}
