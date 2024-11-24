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
        _: usize,
        S: String,
    };

    let vs = S.split('/').collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..vs.len() - 1 {
        let c1 = vs[i].chars().rev().take_while(|c| c == &'1').count();
        let c2 = vs[i + 1].chars().take_while(|c| c == &'2').count();
        chmax!(ans, c1.min(c2) * 2 + 1);
    }
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
