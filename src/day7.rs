use std::rc::Rc;
use std::cell::RefCell;

struct FSNode {
    name: String,
    size: usize,

    parent: Option<Rc<RefCell<FSNode>>>,
    children: Vec<Rc<RefCell<FSNode>>>,
}

fn build_fs(cmds: Vec<&str>) -> Rc<RefCell<FSNode>> {
    let root = Rc::new(RefCell::new(FSNode {
        name: "/".to_string(),
        size: 0,
        parent: None,
        children: Vec::new()
    }));
    let mut current = Rc::clone(&root);

    let mut cmd_iter = cmds.iter().skip(1); // Since the root is already created, skip first cd
    while let Some(&cmd) = cmd_iter.next() {

        match &cmd[0..4] {
            "$ ls" => { },
            "$ cd" => {
                let (_, rest) = cmd.split_once(" ").unwrap();
                let (_, target_name) = rest.split_once(" ").unwrap();
                //println!("Change directory to {}", target_name);

                let clone = Rc::clone(&current);
                let node = clone.borrow();
                let target = node.children.iter().find(|c| c.borrow().name == target_name);
                match target {
                    Some(n) => current = Rc::clone(n),
                    None => current = Rc::clone(&node.parent.as_ref().unwrap()),
                };
            },
            _ => {
                let (first, second) = cmd.split_once(" ").unwrap();
                let (name, size) = match first {
                    "dir" => { (second, 0) },
                    _ => { (second, first.parse::<usize>().unwrap()) },
                };

                //println!("New node -> name: {}; size: {}", name, size);

                let child = Rc::new(RefCell::new(FSNode {
                    name: name.to_string(),
                    size,
                    parent: Some(Rc::clone(&current)),
                    children: Vec::new()
                }));

                let mut mut_curr = current.borrow_mut();
                mut_curr.children.push(child);
            },
        }
    }

    _ = calc_dir_sizes(Rc::clone(&root));
    return root;
}

fn calc_dir_sizes(current: Rc<RefCell<FSNode>>) -> usize {
    if current.borrow().children.len() <= 0 {
        return current.borrow().size;
    }
    else {
        let dir_size = current.borrow().children
            .iter()
            .map(|c| calc_dir_sizes(c.to_owned()))
            .sum::<usize>();

        current.borrow_mut().size = dir_size;
        return dir_size;
    }
}

fn debug_fs(root: Rc<RefCell<FSNode>>) {
    debug_fs_impl(root, 0);
}

fn debug_fs_impl(current: Rc<RefCell<FSNode>>, depth: usize) {
    let prefix = (0..depth).map(|_| ' ').collect::<String>();
    let curr = current.borrow();

    if curr.children.len() > 0 {
        println!("{}- {} (dir, size={})", prefix, curr.name, curr.size);

        curr.children.iter()
            .for_each(|c| debug_fs_impl(c.to_owned(), depth+1));
    }
    else {
        println!("{}- {} (file, size={})", prefix, curr.name, curr.size);
    }
}

pub fn main() {
    println!("[Day7] Solutions:");

    let cmds = include_str!("../data/day7.input")
        .lines()
        .collect::<Vec<&str>>();

    let fs = build_fs(cmds);

    println!();
    debug_fs(Rc::clone(&fs));
    println!();

    println!("[Day7] Part 1 => {}", run_part1(Rc::clone(&fs)));
    println!("[Day7] Part 2 => {}", run_part2(Rc::clone(&fs)));

    println!("[Day7] Complete -----------------------");
}

fn list_dir(fs: Rc<RefCell<FSNode>>) -> Vec<Rc<RefCell<FSNode>>> {
    let mut ret = Vec::<Rc<RefCell<FSNode>>>::new();

    list_dir_impl(fs, &mut ret);
    return ret;
}

fn list_dir_impl(fs: Rc<RefCell<FSNode>>, list: &mut Vec<Rc<RefCell<FSNode>>>) {
    if fs.borrow().children.len() > 0 {
        list.push(Rc::clone(&fs));
        fs.borrow().children
            .iter()
            .for_each(|c| list_dir_impl(c.to_owned(), list));
    }
}

fn run_part1(fs: Rc<RefCell<FSNode>>) -> usize {
    return list_dir(fs).iter()
        .filter_map(|n| {
            let curr = n.borrow();
            if curr.children.len() <= 0 {
                None
            }
            else {
                if curr.size > 100_000 {
                    None
                }
                else {
                    Some(curr.size)
                }
            }
        })
        .sum();
}

fn run_part2(fs: Rc<RefCell<FSNode>>) -> usize {
    const SPACE_AVAILABLE: usize = 70_000_000;
    const SPACE_NEEDED: usize = 30_000_000;

    let unused = SPACE_AVAILABLE - fs.borrow().size;
    let missing = SPACE_NEEDED - unused;

    //println!("unused space = {}; needed = {}; missing = {}", unused, SPACE_NEEDED, missing);

    return list_dir(fs).iter()
        .filter_map(|n| {
            let curr = n.borrow();
            if curr.children.len() <= 0 {
                None
            }
            else {
                if curr.size >= missing {
                    Some(curr.size)
                }
                else {
                    None
                }
            }
        })
        .min().unwrap();
}
