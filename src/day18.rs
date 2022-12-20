use std::collections::HashSet;
type Voxel = (i32, i32, i32);

pub fn main() {
    println!("[Day18] Solutions:");

    let voxels = include_str!("../data/day18.example")
        .lines()
        .map(|line| {
            let (x_str, rest) = line.split_once(",").unwrap();
            let (y_str, z_str) = rest.split_once(",").unwrap();

            (
                x_str.parse::<i32>().unwrap(),
                y_str.parse::<i32>().unwrap(),
                z_str.parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<Voxel>>();

    let droplet = voxels.iter().fold(HashSet::new(), |mut drop, &v| {
        drop.insert(v);
        drop
    });

    println!("[Day18] Part 1 => {}", run_part1(&voxels, &droplet));
    println!("[Day18] Part 2 => {}", run_part2(&voxels, &droplet));

    println!("[Day18] Complete -----------------------");
}

fn run_part1(voxels: &Vec<Voxel>, drop: &HashSet<Voxel>) -> u32 {
    let mut count = 0;
    let surround = vec![
        (-1, 0, 0), (1, 0, 0),
        (0, -1, 0), (0, 1, 0),
        (0, 0, -1), (0, 0, 1),
    ];

    voxels.iter().for_each(|v| {
        let (vx, vy, vz) = v;

        surround.iter()
            .map(|&(sx, sy, sz)| {
                (vx + sx, vy + sy, vz + sz)
            })
            .for_each(|c| {
                if let None = drop.get(&c) {
                    count += 1;
                }
            });
    });

    return count;
}

fn run_part2(voxels: &Vec<Voxel>, drop: &HashSet<Voxel>) -> u32 {
    println!("{} voxels", voxels.len());
    println!("{} drop cubes", drop.len());

    return 0;
}
