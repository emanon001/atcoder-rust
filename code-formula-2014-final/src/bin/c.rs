#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn solve() {
    let s = read_line();
    let re = Regex::new(r"@(\w+)").unwrap();
    let mut res = BTreeSet::new();
    for cap in re.captures_iter(&s) {
        let name = cap[1].to_string();
        res.insert(name);
    }
    for name in res {
        println!("{}", name);
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
