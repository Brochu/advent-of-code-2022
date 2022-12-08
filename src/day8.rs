struct Map {
    trees: Vec<u32>,
    n: usize,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let n = input.lines().next().unwrap().len();

        return Self {
            n,
            trees: input.lines()
            .flat_map(|line| {
                line.chars().map(|ch| ch.to_digit(10).unwrap())
            })
            .collect::<Vec<u32>>(),
        }
    }

    fn coords_to_idx(&self, x: usize, y: usize) -> usize {
        return (y * self.n) + x;
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        return self.trees[self.coords_to_idx(x, y)];
    }

    pub fn inner_range(&self) -> Vec<(usize, usize)> {
        return (1..self.n-1).flat_map(|x| {
            (1..self.n-1).map(move |y| (x, y))
        })
        .collect::<Vec<(usize, usize)>>();
    }

    pub fn vision_range(&self, x: usize, y: usize) -> Vec<Vec<(usize, usize)>> {
        return vec![ // Up, Down, Left, Right
            (0..y).map(|yc| (x, yc)).collect::<Vec<(usize, usize)>>(),
            (y+1..self.n).map(|yc| (x, yc)).collect::<Vec<(usize, usize)>>(),
            (0..x).map(|xc| (xc, y)).collect::<Vec<(usize, usize)>>(),
            (x+1..self.n).map(|xc| (xc, y)).collect::<Vec<(usize, usize)>>(),
        ];
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut output = String::from("");
        output.push_str(format!("Map of size {}\n", self.n).as_str());

        self.trees[..].chunks(self.n.into())
            .for_each(|chunk| output.push_str(format!("{:?}\n", chunk).as_str()));

        return output;
    }
}

pub fn main() {
    println!("[Day8] Solutions:");

    let map = Map::new(include_str!("../data/day8.input"));
    //println!("{}", map.to_string());
    //println!("{}", map.get(4, 4));
    //map.inner_range().iter()
    //    .for_each(|coord| println!("{:?}", coord));
    //map.vision_range(3, 3).iter()
    //    .for_each(|coord| println!("{:?}", coord));
    
    println!("[Day8] Part 1 => {}", run_part1(&map));
    println!("[Day8] Part 2 => {}", run_part2(&map));

    println!("[Day8] Complete -----------------------");
}

fn run_part1(map: &Map) -> usize {
    let outside = (map.n-1) * 4;

    return map.inner_range()
        .iter()
        .map(|(x, y)|{
            let curr_height = map.get(*x, *y);
            map.vision_range(*x, *y)
                .iter()
                .fold(false, |visible, v| {
                    visible || v.iter().all(|(x, y)| map.get(*x, *y) < curr_height)
                })
        })
        .filter(|r| *r)
        .count() + outside;
}

fn run_part2(_map: &Map) -> usize {
    return 0;
}
