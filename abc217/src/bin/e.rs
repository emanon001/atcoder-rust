#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        q: usize,
    };

    let mut front_map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut back = VecDeque::new();
    for _ in 0..q {
        input! { kind: usize };
        match kind {
            1 => {
                input! { x: usize };
                back.push_back(x);
            }
            2 => {
                if !front_map.is_empty() {
                    let (&v, &c) = front_map.iter().next().unwrap();
                    println!("{}", v);
                    let next_c = c - 1;
                    front_map.insert(v, next_c);
                    if next_c == 0 {
                        front_map.remove(&v);
                    }
                } else {
                    let v = back.pop_front().unwrap();
                    println!("{}", v);
                }
            }
            3 => {
                for &v in &back {
                    *front_map.entry(v).or_insert(0) += 1;
                }
                back = VecDeque::new();
            }
            _ => unreachable!()
        }
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
