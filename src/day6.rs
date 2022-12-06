pub fn main() {
    println!("[Day6] Solutions:");

    let streams = include_str!("../data/day6.input")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day6] Part 1 => {}", run_part1(&streams));
    println!("[Day6] Part 2 => {}", run_part2(&streams));

    println!("[Day6] Complete -----------------------");
}

fn valid_start(window: &[char]) -> bool {
    let idx_map = window.iter()
        .map(|&c| c as u8 - b'a' as u8);

    let mut bitfield: u32 = 0;

    for idx in idx_map {
        let mask: u32 = 1 << idx;

        if (bitfield & mask) > 0 {
            return false;
        }

        bitfield |= mask;
    }
    return true;
}

fn run_part1(streams: &Vec<&str>) -> usize {
    let starts = streams.iter()
        .map(|stream| {
            stream.chars().collect::<Vec<char>>()
                .windows(4).enumerate()
                .fold((false, 0), |res, win| {
                    match  res.0 {
                        true => res,
                        false => {
                            (valid_start(win.1), win.0 + 4)
                        },
                    }
                })
        })
        .map(|res| res.1)
        .collect::<Vec<usize>>();

    println!("{:?}", starts);
    return starts[0];
}

fn run_part2(streams: &Vec<&str>) -> usize {
    let starts = streams.iter()
        .map(|stream| {
            stream.chars().collect::<Vec<char>>()
                .windows(14).enumerate()
                .fold((false, 0), |res, win| {
                    match  res.0 {
                        true => res,
                        false => {
                            (valid_start(win.1), win.0 + 14)
                        },
                    }
                })
        })
        .map(|res| res.1)
        .collect::<Vec<usize>>();

    println!("{:?}", starts);
    return starts[0];
}
