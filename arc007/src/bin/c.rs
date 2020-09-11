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
        s: Chars
    };

    let len = s.len();
    let mut res = 1_usize << 30;
    for bits in 0..2 << len {
        let mut count = 1;
        let mut state = s.repeat(2);
        for i in 0..len {
            if (bits >> i) & 1 == 0 {
                continue;
            }
            count += 1;
            for j in 0..len {
                if s[j] == 'o' {
                    state[i + j] = 'o';
                }
            }
        }
        let mut is_ok = false;
        let mut l = -1;
        for j in 0_isize..state.len() as isize {
            if state[j as usize] == 'o' {
                if j - l >= len as isize {
                    is_ok = true;
                    break;
                }
            } else {
                l = j;
            }
        }
        if is_ok && count < res {
            res = count;
        }
    }
    println!("{}", res);
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
