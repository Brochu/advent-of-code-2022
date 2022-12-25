pub fn main() {
    println!("[Day25] Solutions:");

    let snafu_nums = include_str!("../data/day25.example")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day25] Part 1 => Number: {}", run_part1(&snafu_nums));
    //println!("[Day25] Part 2 => {}", run_part2());

    println!("[Day25] Complete -----------------------");
}

fn snafu_to_dec(snafu: &str) -> i64 {
    let dec = snafu.chars().rev().enumerate()
        .fold(0 as i64, |acc, (i, d)| {
            let val = match d {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid snafu digit"),
            };
            let pos = (5 as i64).pow(i as u32);

            acc + (val * pos)
        });

    println!("Converting from (snafu): '{}' to (decimal): {}", snafu, dec);
    return dec;
}

fn dec_to_snafu(mut dec: i64) -> String {
    let mut base5 = Vec::new();
    while dec > 0 {
        base5.push((dec % 5) as u8);
        dec /= 5;
    }

    let (snafu, rest) = base5.iter()
        .fold((String::new(), 0), |(mut snafu, mut rest), num| {
            (snafu, rest)
        });

    return String::from("''");
}

fn run_part1(snafu_nums: &Vec<&str>) -> String {
    let dec_sum: i64 = snafu_nums.iter()
        .map(|n| snafu_to_dec(n))
        .sum();

    println!();
    println!("Sum (in decimal): {}", dec_sum);

    return dec_to_snafu(dec_sum);
}

//fn run_part2() -> i64 {
//    return 0;
//}
