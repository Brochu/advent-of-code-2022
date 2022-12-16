use std::collections::HashMap;
use std::fmt::Debug;

struct Network {
    valves: Vec<Valve>,
    lut: HashMap<String, usize>,
}

fn create_network(input: &str) -> Network {
    let valves = Vec::new();
    let lut = HashMap::new();

    input.lines()
        .for_each(|line| println!("{}", line));

    return Network { valves, lut }
}

struct Valve {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vavle {}; Flow {}; tunnels {:?}", self.name, self.flow, self.tunnels)
    }
}

pub fn main() {
    println!("[Day16] Solutions:");

    let network = create_network(include_str!("../data/day16.example"));

    println!("[Day16] Part 1 => {}", run_part1(&network));
    //println!("[Day16] Part 2 => {}", run_part2(&lines));

    println!("[Day16] Complete -----------------------");
}

fn run_part1(net: &Network) -> u32 {
    println!("Network:\n\tVavles: {:?}\n\tLUT: {:?}", net.valves, net.lut);
    return 0;
}

//fn run_part2(network: &Network) -> usize {
//    return lines.len();
//}
