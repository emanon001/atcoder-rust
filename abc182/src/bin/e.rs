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
        h: usize, w: usize, n: usize, m: usize,
        abv: [(Usize1, Usize1); n],
        cdv: [(Usize1, Usize1); m],
    };

    let hikari = abv.into_iter().collect::<HashSet<_>>();
    let blocks = cdv.into_iter().collect::<HashSet<_>>();
    let mut used = vec![vec![false; w]; h];
    // 横方向
    for i in 0..h {
        let mut l = 0;
        let mut ok = false;
        for j in 0..w {
            if blocks.contains(&(i, j)) {
                if ok {
                    for jj in l..j {
                        used[i][jj] = true;
                    }
                }
                ok = false;
                l = j + 1;
            } else {
                if hikari.contains(&(i, j)) {
                    ok = true;
                }
                if ok {
                    if j == w - 1 {
                        for jj in l..=j {
                            used[i][jj] = true;
                        }
                    }
                }
            }
        }
    }
    // 縦方向
    for j in 0..w {
        let mut t = 0;
        let mut ok = false;
        for i in 0..h {
            if blocks.contains(&(i, j)) {
                if ok {
                    for ii in t..i {
                        used[ii][j] = true;
                    }
                }
                ok = false;
                t = i + 1;
            } else {
                if hikari.contains(&(i, j)) {
                    ok = true;
                }
                if ok {
                    if i == h - 1 {
                        for ii in t..=i {
                            used[ii][j] = true;
                        }
                    }
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if used[i][j] {
                res += 1;
            }
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
