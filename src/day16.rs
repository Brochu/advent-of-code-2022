use std::fmt::Display;
use std::collections::HashMap;
use std::collections::HashSet;

struct Network {
    valves: Vec<Valve>,
    lut: HashMap<String, usize>,
}

struct Valve {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Valve {}; Flow {}; tunnels {:?}", self.name, self.flow, self.tunnels)
    }
}

#[derive(Clone, Debug)]
enum Action {
    Move(String),
    Open(String),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(name) => write!(f, "Move({})", name),
            Action::Open(name) => write!(f, "Open({})", name),
        }
    }
}

fn create_network(input: &str) -> Network {
    let (valves, lut) = input.lines()
        .fold((Vec::new(), HashMap::new()), |(mut v, mut l), line| {
            let (valve_str, tunnels_str) = line.split_once("; ").unwrap();
            let (valve_str, flow_str) = valve_str.split_once(" has flow rate=").unwrap();
            let (_, tunnels_str) = tunnels_str.split_once(" to ").unwrap();

            let name = String::from(valve_str.strip_prefix("Valve ").unwrap());
            let flow = flow_str.parse::<u32>().unwrap();
            let tunnels = tunnels_str.split(" ")
                .skip(1)
                .map(|t| t.chars().take(2).collect::<String>() )
                .collect::<Vec<String>>();

            l.insert(name.clone(), v.len());
            v.push(Valve { name, flow, tunnels });

            (v, l)
        });

    return Network { valves, lut }
}

fn _show_network(net: &Network) {
    println!("Network:");

    println!("Valves:");
    net.valves.iter().for_each(|v| println!("\t{v}"));

    println!("LUT:");
    net.lut.iter().for_each(|(name, idx)| println!("\t({}) -> {}", name, idx));
}

pub fn main() {
    println!("[Day16] Solutions:");

    let network = create_network(include_str!("../data/day16.example"));

    println!("[Day16] Part 1 => {}", run_part1(&network));
    //println!("[Day16] Part 2 => {}", run_part2(&lines));

    println!("[Day16] Complete -----------------------");
}

fn find_best_actions(net: &Network,
                     mut opened: HashSet<String>,
                     mut plan: Vec<Action>,
                     depth: u8,
                     pressure: u32) -> (Vec<Action>, u32) {
    if depth >= 30 {
        return (plan, pressure);
    }

    plan.push(Action::Move(String::from("AA")));
    opened.insert(String::from("AA"));
    return find_best_actions(net, opened, plan, depth+1, pressure);
}

fn run_part1(net: &Network) -> u32 {
    //show_network(net);

    let (plan, pressure) = find_best_actions(net, HashSet::new(), Vec::new(), 0, 0);
    plan.iter().enumerate().for_each(|(idx, s)| println!("[{}]: {}", idx, s));
    println!("Pressure: {}", pressure);

    return 0;
}

//fn run_part2(network: &Network) -> usize {
//    return lines.len();
//}
