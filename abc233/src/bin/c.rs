#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, j: usize, perm: &mut VecDeque<i64>, x: i64, lv: &[Vec<i64>], res: &mut i64) {
    perm.push_back(lv[i][j]);
    if perm.len() == lv.len() {
        let mut cur = 1_i64;
        for ii in 0..perm.len() {
            let checked_cur = cur.checked_mul(perm[ii]);
            if checked_cur.is_none() {
                break;
            }
            cur = checked_cur.unwrap();
            if ii + 1 == lv.len() && cur == x {
                *res += 1;
            }
        }
    } else {
        for jj in 0..lv[i + 1].len() {
            dfs(i + 1, jj, perm, x, lv, res);
        }
    }
    perm.pop_back();
}

fn solve() {
    input! {
        n: usize, x: i64,
        lv: [[i64]; n]
    };

    let mut res = 0;
    let mut perm = VecDeque::new();
    for j in 0..lv[0].len() {
        dfs(0, j, &mut perm, x, &lv, &mut res);
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
