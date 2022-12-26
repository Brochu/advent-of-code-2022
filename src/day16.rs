use std::fmt::Display;
use std::collections::HashMap;

type Cache = HashMap::<(usize, u64, u64), u64>;

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
    println!("[Day16] Part 2 => {}", run_part2(&network));

    println!("[Day16] Complete -----------------------");
}

fn calc_pressure(net: &Network, open_field: u64) -> u64 {
    return net.valves.iter()
        .filter_map(|v| {
            if (open_field & (1 << net.lut[&v.name])) > 0 { Some(v.flow as u64) }
            else { None }
        })
        .sum();
}

fn best_pressure(net: &Network, pos: usize, open_field: u64, time: u64, cache: &mut Cache) -> u64 {
    if let Some(&val) = cache.get(&(pos, open_field, time)) {
        return val;
    }

    if time == 1 {
        let res = calc_pressure(net, open_field);
        cache.insert((pos, open_field, time), res);
        return res;
    }
    else {
        let v = &net.valves[pos];
        let should_open = v.flow > 0;
        let on = open_field & (1 << pos) > 0;

        let mut options = Vec::<u64>::new();

        if !on && should_open {
            options.push(best_pressure(net, pos, open_field | (1 << pos), time-1, cache));
        }

        v.tunnels.iter()
            .map(|t| net.lut[t])
            .for_each(|idx| {
                options.push(best_pressure(net, idx, open_field, time-1, cache));
            });

        let res = match options.iter().max() {
            Some(&val) => val,
            None => best_pressure(net, pos, open_field, time-1, cache),
        } + calc_pressure(net, open_field);

        cache.insert((pos, open_field, time), res);
        return res;
    }
}

fn best_pressure_part2(net: &Network, my_pos: usize, ele_pos: usize, open_field: u64, time: u64, cache: &mut Cache) -> u64{
    if let Some(&val) = cache.get(&(my_pos, open_field, time)) {
        return val;
    }

    if time == 1 {
        let res = calc_pressure(net, open_field);
        cache.insert((my_pos, open_field, time), res);
        return res;
    }
    else {
        let v = &net.valves[my_pos];
        let should_open = v.flow > 0;
        let on = open_field & (1 << my_pos) > 0;

        let mut options = Vec::<u64>::new();

        if !on && should_open {
            options.push(best_pressure_part2(net, my_pos, ele_pos, open_field | (1 << my_pos), time-1, cache));
        }

        v.tunnels.iter()
            .map(|t| net.lut[t])
            .for_each(|idx| {
                options.push(best_pressure_part2(net, idx, ele_pos, open_field, time-1, cache));
            });

        let res = match options.iter().max() {
            Some(&val) => val,
            None => best_pressure_part2(net, my_pos, ele_pos, open_field, time-1, cache),
        } + calc_pressure(net, open_field);

        cache.insert((my_pos, open_field, time), res);
        return res;
    }
}

fn run_part1(net: &Network) -> u64 {
    let start_pos: usize = net.lut[&String::from("AA")];
    const INIT_FIELD: u64 = 0;
    const INIT_TIME: u64 = 30;
    let mut cache = Cache::new();

    return best_pressure(net, start_pos, INIT_FIELD, INIT_TIME, &mut cache);
}

fn run_part2(net: &Network) -> u64 {
    //println!("{}", net);

    let start_pos: usize = net.lut[&String::from("AA")];
    const INIT_FIELD: u64 = 0;
    const INIT_TIME: u64 = 26;
    let mut cache = Cache::new();

    return best_pressure_part2(net, start_pos, start_pos, INIT_FIELD, INIT_TIME, &mut cache);
}
