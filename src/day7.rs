struct FS {
    nodes: Vec<FSNode>,
    tree: Vec<FSRef>,
}

struct FSNode {
    name: String,
    size: usize,
}

struct FSRef {
    index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

fn build_fs(cmds: Vec<&str>) -> FS {
    let mut nodes: Vec<FSNode> = Vec::new();
    nodes.push(FSNode { name: "/".to_string(), size: 0 });

    let mut tree: Vec<FSRef> = Vec::new();
    tree.push(FSRef { index: 0, parent: None, children: Vec::new() });
    let mut current_idx = 0;

    let mut cmd_iter = cmds.iter().skip(1); // Since the root is already created, skip first cd
    while let Some(&cmd) = cmd_iter.next() {

        match &cmd[0..4] {
            "$ ls" => { },
            "$ cd" => {
                let (_, rest) = cmd.split_once(" ").unwrap();
                let (_, target_name) = rest.split_once(" ").unwrap();
                //println!("Change directory to {}", target_name);

                match tree[current_idx].children.iter().find(|&&i| nodes[i].name == target_name) {
                    Some(&i) => current_idx = i,
                    None => current_idx = tree[current_idx].parent.unwrap(),
                }
            },
            _ => {
                let (first, second) = cmd.split_once(" ").unwrap();
                let (name, size) = match first {
                    "dir" => { (second, 0) },
                    _ => { (second, first.parse::<usize>().unwrap()) },
                };

                //println!("New node -> name: {}; size: {}", name, size);
                let index = nodes.len();
                let tree_idx = tree.len();

                nodes.push(FSNode { name: name.to_string(), size });
                tree.push(FSRef { index, parent: Some(current_idx), children: Vec::new() });
                tree[current_idx].children.push(tree_idx);
            },
        }
    }

    calc_dir_sizes(&mut nodes, &tree);
    return FS { nodes, tree };
}

fn calc_dir_sizes(nodes: &mut Vec<FSNode>, tree: &Vec<FSRef>) {
    calc_dir_sizes_impl(nodes, tree, 0);
}

fn calc_dir_sizes_impl(nodes: &mut Vec<FSNode>, tree: &Vec<FSRef>, cur_idx: usize) {
    let curr_ref = &tree[cur_idx];

    if curr_ref.children.len() <= 0 {
        return
    }
    else {
        curr_ref.children
            .iter()
            .for_each(|&c| calc_dir_sizes_impl(nodes, tree, c));

        let dir_size = curr_ref.children
            .iter()
            .map(|&c| {
                let fs_ref = &tree[c];
                let other = &nodes[fs_ref.index];
                other.size
            })
            .sum::<usize>();

        nodes[curr_ref.index].size = dir_size;
    }
}

fn debug_fs(fs: &FS) {
    debug_ref(&fs.tree[0], &fs, 0);
}

fn debug_ref(current: &FSRef, fs: &FS, depth: usize) {
    let node: &FSNode = &fs.nodes[current.index];

    let prefix = (0..depth).map(|_| ' ').collect::<String>();
    if current.children.len() > 0 {
        println!("{}- {} (dir, size={})", prefix, node.name, node.size);

        current.children.iter()
            .for_each(|&i| debug_ref(&fs.tree[i], fs, depth+1));
    }
    else {
        println!("{}- {} (file, size={})", prefix, node.name, node.size);
    }
}

pub fn main() {
    println!("[Day7] Solutions:");

    let cmds = include_str!("../data/day7.input")
        .lines()
        .collect::<Vec<&str>>();

    let fs = build_fs(cmds);

    println!();
    debug_fs(&fs);
    println!();

    println!("[Day7] Part 1 => {}", run_part1(&fs));
    println!("[Day7] Part 2 => {}", run_part2(&fs));

    println!("[Day7] Complete -----------------------");
}

fn run_part1(fs: &FS) -> usize {
    return fs.tree.iter()
        .filter_map(|n| {
            if n.children.len() <= 0 {
                None
            }
            else {
                let node = &fs.nodes[n.index];
                if node.size > 100_000 {
                    None
                }
                else {
                    Some(node.size)
                }
            }
        })
        .sum();
}

fn run_part2(fs: &FS) -> usize {
    const SPACE_AVAILABLE: usize = 70_000_000;
    const SPACE_NEEDED: usize = 30_000_000;

    let root = &fs.nodes[0];
    let unused = SPACE_AVAILABLE - root.size;
    let missing = SPACE_NEEDED - unused;

    //println!("unused space = {}; needed = {}; missing = {}", unused, SPACE_NEEDED, missing);

    return fs.tree.iter()
        .filter_map(|n| {
            if n.children.len() <= 0 {
                None
            }
            else {
                let node = &fs.nodes[n.index];
                if node.size >= missing {
                    Some(node.size)
                }
                else {
                    None
                }
            }
        })
        .min().unwrap();
}
