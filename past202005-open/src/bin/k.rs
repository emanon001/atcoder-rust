#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

// container
#[derive(Clone, Copy, Debug)]
struct C {
    up: Option<usize>,
    down: Option<usize>,
}

fn solve() {
    input! {
        n: usize, q: usize,
        queries: [(Usize1, Usize1, Usize1); q]
    };
    let mut containers = vec![
        C {
            up: None,
            down: None
        };
        n
    ];
    let mut tops = (0..n).map(|i| Some(i)).collect::<Vec<_>>();
    for (f, t, x) in queries {
        let cur_x_down = containers[x].down;

        // 移動先の机を更新
        if let Some(t_top) = tops[t] {
            // 机にコンテナが存在する
            containers[t_top].up = Some(x);
            containers[x].down = Some(t_top);
        } else {
            // 机にコンテナが存在しない
            containers[x].down = None;
        }
        tops[t] = tops[f];

        // 移動元の机を更新
        if let Some(x_down) = cur_x_down {
            // 机にコンテナが存在する
            containers[x_down].up = None;
        }
        tops[f] = cur_x_down;
    }

    let mut res = vec![0; n];
    for desk in 0..n {
        let mut con = tops[desk];
        while let Some(c) = con {
            res[c] = desk + 1;
            con = containers[c].down;
        }
    }
    for desk in res {
        println!("{}", desk);
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
