use std::fmt::Display;
use std::collections::HashMap;

struct Network {
    valves: Vec<Valve>,
    lut: HashMap<String, usize>,
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("Valve:\n");
        self.valves.iter().for_each(|v| { output = format!("{}\t{}\n", output, v); });

        output = format!("{}LUT:\n", output);
        self.lut.iter().for_each(|(name, idx)| { output = format!("{}\t({}) -> {}\n", output, name, idx); });

        write!(f, "{}", output)
    }
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

pub fn main() {
    println!("[Day16] Solutions:");

    let network = create_network(include_str!("../data/day16.example"));

    println!("[Day16] Part 1 => {}", run_part1(&network));
    //println!("[Day16] Part 2 => {}", run_part2(&lines));

    println!("[Day16] Complete -----------------------");
}

fn calc_pressure(net: &Network, open_field: u32) -> u32 {
    return net.valves.iter()
        .filter_map(|v| {
            if (open_field & (1 << net.lut[&v.name])) > 0 { Some(v.flow) }
            else { None }
        })
        .sum();
}

fn best_pressure(net: &Network, pos: usize, open_field: u32, time: u32) -> u32 {
    if time == 0 {
        return calc_pressure(net, open_field);
    }
    else {
        let v = &net.valves[pos];
        let on = open_field & (1 << pos) > 0;
        println!("Current position: {} (on? {})", v, on);
        println!("\tOpen field: {:#034b}; At time {}", open_field, time);

        // All possible options
        let mut options = Vec::<u32>::new();

        // If not already on, turn on current valve
        if !on {
            options.push(best_pressure(net, pos, open_field | (1 << pos), time - 1));
        }

        // Visit any neighbours
        //TODO
        //v.tunnels.iter()
        //    .map(|t| net.lut[t])
        //    .for_each(|idx| println!("\t{}", net.valves[idx]));

        let best = options.iter().max();
        return match best {
            Some(&val) => val,
            None => best_pressure(net, pos, open_field, time-1),
        };
    }
}

fn run_part1(net: &Network) -> u32 {
    //println!("{}", net);

    //let bitset = net.valves.iter()
    //    .filter_map(|v| {
    //        if v.flow > 0 { Some(net.lut[&v.name]) }
    //        else { None }
    //    })
    //    .fold(0 as u64, |acc, idx| {
    //        acc | (1 << idx)
    //    });

    let start_pos: usize = net.lut[&String::from("AA")];
    const INIT_FIELD: u32 = 0;
    const INIT_TIME: u32 = 30;
    return best_pressure(net, start_pos, INIT_FIELD, INIT_TIME);
}

//fn run_part2(network: &Network) -> usize {
//    return lines.len();
//}
