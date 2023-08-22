#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
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

fn solve() {
    input! {
        _: usize, m: usize, h: isize, k: isize,
        s: Chars,
        xyv: [(isize, isize); m]
    };

    let mut xy_set = xyv.into_iter().collect::<HashSet<_>>();

    let mut cur_pos = (0, 0);
    let mut cur_h = h;
    for ch in s {
        cur_h -= 1;
        cur_pos = match ch {
            'R' => (cur_pos.0 + 1, cur_pos.1),
            'L' => (cur_pos.0 - 1, cur_pos.1),
            'U' => (cur_pos.0, cur_pos.1 + 1),
            'D' => (cur_pos.0, cur_pos.1 - 1),
            _ => unreachable!(),
        };
        if cur_h < 0 {
            println!("No");
            return;
        }
        if xy_set.contains(&cur_pos) && cur_h < k {
            xy_set.remove(&cur_pos);
            cur_h = k;
        }
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
