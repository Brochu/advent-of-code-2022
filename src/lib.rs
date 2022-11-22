use std::env;
use std::fs;

pub fn read_file(folder: &str, day: u8) -> String {
    let path = env::current_dir().unwrap()
        .join(folder)
        .join(format!("day{}.txt", day));
    println!("[LIB] Reading file at {}", path.to_str().unwrap());

    let content = fs::read_to_string(&path);
    let err_str = format!("[LIB] Could not read file at {}", path.to_str().unwrap());

    return content.expect(&err_str);
}
