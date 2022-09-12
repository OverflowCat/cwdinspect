use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("cwd: {:?}", env::current_dir().unwrap().to_str().unwrap());
    println!("exe: {:?}", env::current_exe().unwrap().to_str().unwrap());
    for file in fs::read_dir("./").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}
