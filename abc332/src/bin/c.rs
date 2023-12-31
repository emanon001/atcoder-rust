#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        _: usize, M: usize,
        S: Chars
    };

    let mut muji = M;
    let mut used_muji = 0;
    let mut logo: usize = 0;
    let mut used_logo = 0;
    for ch in S {
        match ch {
            '0' => {
                muji += used_muji;
                used_muji = 0;
                logo += used_logo;
                used_logo = 0;
            }
            '1' => {
                if muji > 0 {
                    used_muji += 1;
                    muji -= 1;
                    continue;
                }

                if logo == 0 {
                    logo += 1;
                }
                logo -= 1;
                used_logo += 1;
            }
            '2' => {
                if logo == 0 {
                    logo += 1;
                }
                logo -= 1;
                used_logo += 1;
            }
            _ => unreachable!(),
        }
    }
    let ans = muji + used_muji + logo + used_logo - M;
    println!("{}", ans);
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
