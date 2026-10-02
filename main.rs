use std::io::{self, BufRead};

// TODO (what-is-vpn): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        println!("{}", l.parse::<u32>().unwrap() + 56);
    }
}
