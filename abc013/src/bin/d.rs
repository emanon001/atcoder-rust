#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn goal(i: usize, j: usize, m: usize, lines: &[BTreeSet<(usize, usize)>]) -> usize {
    let mut i = i;
    let mut j = j;
    while i < m {
        if let Some((ii, dir)) = lines[j].range((i, 0)..).next() {
            i = *ii + 1;
            j = if *dir == 0 { j + 1 } else { j - 1 };
        } else {
            i = m;
        }
    }
    j
}

fn solve() {
    input! {
        n: usize, m: usize, d: usize,
        av: [Usize1; m]
    };

    let mut lines = vec![BTreeSet::new(); n];
    for (i, a) in av.into_iter().enumerate() {
        lines[a].insert((i, 0));
        if a + 1 < n {
            lines[a + 1].insert((i, 1));
        }
    }
    let mut to = vec![vec![0; n]; 30];
    for j in 0..n {
        let g = goal(0, j, m, &lines);
        to[0][j] = g;
    }
    for i in 0..29 {
        for j in 0..n {
            let k = to[i][to[i][j]];
            to[i + 1][j] = k;
        }
    }
    for i in 0..n {
        let mut rest = d;
        let mut cur = i;
        while rest > 0 {
            let mut c = 0;
            while 2.pow(c + 1) <= rest {
                c += 1;
            }
            let c = c as usize;
            cur = to[c][cur];
            rest -= 2.pow(c as u32);
        }
        println!("{}", cur + 1);
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
