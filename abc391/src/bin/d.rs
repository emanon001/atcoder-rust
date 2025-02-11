#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, W: usize,
        XY: [(Usize1, Usize1); N],
        Q: usize,
        TA: [(Usize1, Usize1); Q],
    };

    let mut cols = vec![vec![]; W];
    for &(x, y) in &XY {
        cols[x].push(y);
    }
    for i in 0..W {
        cols[i].sort();
    }

    let mut pos_to_i = HashMap::new();
    for (i, (x, y)) in XY.into_iter().enumerate() {
        pos_to_i.insert((x, y), i);
    }

    let mut disappear_times = HashMap::new();
    for i in 0..N {
        let has_all = (0..W).all(|j| cols[j].len() >= i + 1);
        if !has_all {
            break;
        }
        let mut max_y = 0;
        for j in 0..W {
            chmax!(max_y, cols[j][i]);
        }
        for j in 0..W {
            disappear_times.insert(pos_to_i[&(j, cols[j][i])], max_y);
        }
    }

    for (t, a) in TA {
        let exists = match disappear_times.get(&a) {
            Some(&dt) => dt > t,
            None => true,
        };
        let ans = if exists { "Yes" } else { "No" };
        println!("{}", ans);
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
