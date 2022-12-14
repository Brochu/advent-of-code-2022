use std::cmp::Ordering;
use serde_json::Value;
type Pair = (Value, Value);

fn parse_pair(pair: (&str, &str)) -> Pair {
    let (l, r) = pair;

    return (
        serde_json::from_str(l).unwrap(),
        serde_json::from_str(r).unwrap(),
    );
}

fn parse_packet(packet: &str) -> Value {
    return serde_json::from_str(packet).unwrap();
}

pub fn main() {
    println!("[Day13] Solutions:");

    let input = include_str!("../data/day13.input");
    let pairs = input.split("\r\n\r\n")
        .map(|s| { parse_pair(s.split_once("\r\n").unwrap()) })
        .collect::<Vec<Pair>>();

    println!("[Day13] Part 1 => {}", run_part1(&pairs));

    let mut packets = input.lines()
        .filter_map(|line| if line.len() != 0 { Some(parse_packet(line)) } else { None })
        .collect::<Vec<Value>>();

    println!("[Day13] Part 2 => {}", run_part2(&mut packets,
                                               serde_json::from_str("[[2]]").unwrap(),
                                               serde_json::from_str("[[6]]").unwrap()));

    println!("[Day13] Complete -----------------------");
}

fn compare_pairs(pair: &Pair) -> Ordering {
    return match pair {
        (Value::Number(lnum), Value::Number(rnum)) => {
            let l = lnum.as_u64().unwrap();
            let r = rnum.as_u64().unwrap();
            l.cmp(&r)
        },
        (Value::Array(llist), Value::Array(rlist)) => {
            let ordered = llist.iter().zip(rlist.iter())
                .fold(Ordering::Equal, |ord, (l, r)| {
                    match ord {
                        Ordering::Equal => {
                            compare_pairs(&(l.to_owned(), r.to_owned()))
                        },
                        _ => ord,
                    }
                });

            match ordered {
                Ordering::Equal => llist.len().cmp(&rlist.len()),
                _ => ordered
            }
        },
        (Value::Number(l), Value::Array(rlist)) => {
            compare_pairs(&(
                Value::Array(vec![Value::Number(l.to_owned())]),
                Value::Array(rlist.to_vec()),
            ))
        },
        (Value::Array(llist), Value::Number(r)) => {
            compare_pairs(&(
                Value::Array(llist.to_vec()),
                Value::Array(vec![Value::Number(r.to_owned())]),
            ))
        },
        _ => panic!("[Day13] Invalid paring for compare"),
    };
}

fn run_part1(pairs: &Vec<Pair>) -> usize {
    return pairs.iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            match compare_pairs(pair) {
                Ordering::Less => Some(i+1),
                Ordering::Greater => None,
                Ordering::Equal => panic!("[Day13] Should not be equal"),
            }
        })
        .sum();
}

// Implementation of the Heap's Algorithm to compute permutations
//fn packets_order_impl(k: usize, mut packets: Vec<usize>, perms: &mut Vec<Vec<usize>>) {
//    if k == 1 {
//        perms.push(packets);
//    }
//    else {
//        packets_order_impl(k - 1, packets.clone(), perms);
//
//        for i in 0..k-1 {
//            if k % 2 == 0 {
//                packets.swap(i, k-1);
//            }
//            else {
//                packets.swap(0, k-1);
//            }
//
//            packets_order_impl(k-1, packets.clone(), perms);
//        }
//    }
//}

//fn packets_order(packets: &Vec<usize>) -> Vec<Vec<usize>> {
//    let mut perms = Vec::<Vec<usize>>::new();
//    packets_order_impl(packets.len(), packets.clone(), &mut perms);
//
//    return perms;
//}

fn run_part2(packets: &mut Vec<Value>, delim0: Value, delim1: Value) -> usize {
    packets.push(delim0.clone());
    packets.push(delim1.clone());

    packets.sort_unstable_by(|l, r| compare_pairs(&(l.to_owned(), r.to_owned())));
    //packets.iter().for_each(|p| println!("{}", p));
    //println!();

    return packets.iter()
        .enumerate()
        .filter(|&(_, val)| val == &delim0 || val == &delim1)
        .map(|(idx, _)| idx + 1)
        .product();
}
