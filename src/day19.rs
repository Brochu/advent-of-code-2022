use std::fmt::Display;

struct BP {
    or_c: u8,
    cl_c: u8,
    ob_or_c: u8, ob_cl_c: u8,
    ge_or_c: u8, ge_ob_c: u8,

    or_m: u8,
    cl_m: u8,
    ob_m: u8,
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
    or: u8, cl: u8, ob: u8, ge: u8,
    or_r: u8, cl_r: u8, ob_r: u8, ge_r: u8,
    time: u8,
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

    let blueprints: Vec<_> = include_str!("../data/day19.example")
        .lines()
        .map(|line| parse_blueprint(line))
        .collect();

    println!("[Day19] Part 1 => {}", run_part1(&blueprints));
    //println!("[Day19] Part 2 => {}", run_part2());

    println!("[Day19] Complete -----------------------");
}

fn check_ore(b: &BP, s: &State) -> (bool, u8) {
    if s.or_r < b.or_m {
        let mut t = 0;
        if s.or < b.or_c { t = (b.or_c - s.or) / s.or_r; }

        return (true, t + 1);
    }
    else {
        return (false, 0);
    }
}

fn check_clay(b: &BP, s: &State) -> (bool, u8) {
    if s.cl_r < b.cl_m {
        let mut t = 0;
        if s.or < b.cl_c { t = (b.cl_c - s.or) / s.or_r; }

        return (true, t + 1);
    }
    else {
        return (false, 0);
    }
}

fn run_part1(bps: &Vec<BP>) -> u32 {
    let mut quality_sum: u32 = 0;

    for (idx, bp) in bps[0..1].iter().enumerate() {
        println!("[{}] {}", idx+1, bp);
        let mut bp_max: u32 = 0;

        let mut stack = vec![ State {
            or: 0, cl: 0, ob: 0, ge: 0,
            or_r: 1, cl_r: 0, ob_r: 0, ge_r: 0,
            time: 0,
        }];

        while let Some(state) = stack.pop() {
            println!("{}", state);

            if state.time == 24 {
                // This branch is over
                if state.ge as u32 > bp_max { bp_max = state.ge as u32 }
                continue;
            }

            let (build_ore, t_ore) = check_ore(bp, &state);
            if build_ore {
                stack.push(State {
                    or: state.or + (state.or_r * t_ore) - bp.or_c,
                    cl: state.cl + (state.cl_r & t_ore),
                    ob: state.ob + (state.ob_r & t_ore),
                    ge: state.ge + (state.ge_r & t_ore),
                    or_r: state.or_r + 1, cl_r: state.cl_r, ob_r: state.ob_r, ge_r: state.ge_r,
                    time: state.time + t_ore,
                })
            }

            let (build_clay, t_clay) = check_clay(bp, &state);
            if build_clay {
                stack.push(State {
                    or: state.or + (state.or_r * t_clay) - bp.cl_c,
                    cl: state.cl + (state.cl_r & t_clay),
                    ob: state.ob + (state.ob_r & t_clay),
                    ge: state.ge + (state.ge_r & t_clay),
                    or_r: state.or_r, cl_r: state.cl_r + 1, ob_r: state.ob_r, ge_r: state.ge_r,
                    time: state.time + t_clay,
                })
            }
        }

        quality_sum += bp_max * (idx+1) as u32
    }

    return quality_sum;
}

//fn run_part2() -> i32 {
//    return 0;
//}
