#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut following = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    a: Usize1, b: Usize1
                };
                following[a].insert(b);
            }
            2 => {
                input! {
                    a: Usize1
                };
                for b in 0..n {
                    if following[b].contains(&a) {
                        following[a].insert(b);
                    }
                }
            }
            3 => {
                input! {
                    a: Usize1
                };
                let mut users = HashSet::new();
                for &x in &following[a] {
                    for &b in &following[x] {
                        if a != b {
                            users.insert(b);
                        }
                    }
                }
                for b in users {
                    following[a].insert(b);
                }
            }
            _ => unreachable!(),
        }
    }
    for i in 0..n {
        let res = (0..n)
            .map(|j| if following[i].contains(&j) { "Y" } else { "N" })
            .join("");
        println!("{}", res);
    }
}
