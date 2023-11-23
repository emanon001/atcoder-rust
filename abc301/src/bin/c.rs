#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    // let mut source = LineSource::new(BufReader::new(stdin()));
    // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        s: Chars,
        t: Chars,
    };

    let mut s_count_map = HashMap::new();
    let mut s_wildcard_count = 0;
    for s_i in s {
        if s_i == '@' {
            s_wildcard_count += 1;
            continue;
        }
        *s_count_map.entry(s_i).or_insert(0) += 1;
    }
    let mut t_wildcard_count = 0;
    let mut t_count_map = HashMap::new();
    for t_i in t {
        if t_i == '@' {
            t_wildcard_count += 1;
            continue;
        }
        let c = *s_count_map.get(&t_i).unwrap_or(&0);
        if c == 0 {
            *t_count_map.entry(t_i).or_insert(0) += 1;
        } else if c == 1 {
            s_count_map.remove(&t_i);
        } else {
            *s_count_map.get_mut(&t_i).unwrap() -= 1;
        }
    }

    let chars = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for (k, c) in s_count_map {
        if !chars.contains(&k) {
            println!("No");
            return;
        }
        if c > t_wildcard_count {
            println!("No");
            return;
        }
        t_wildcard_count -= c;
    }
    for (k, c) in t_count_map {
        if !chars.contains(&k) {
            println!("No");
            return;
        }
        if c > s_wildcard_count {
            println!("No");
            return;
        }
        s_wildcard_count -= c;
    }
    println!("Yes");
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
