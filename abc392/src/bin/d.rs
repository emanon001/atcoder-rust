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
        N: usize,
        K: [[usize]; N],
    };

    let mut counts = vec![vec![0; 10.pow(5) + 1]; N];
    for i in 0..N {
        for &k in &K[i] {
            counts[i][k] += 1;
        }
    }

    let mut ans: num::rational::Ratio<usize> = num::rational::Ratio::new(0, 1);
    for i in 0..N {
        for j in i + 1..N {
            let all_count = K[i].len() * K[j].len();
            let mut count = 0;
            if K[i].len() <= K[j].len() {
                for &k in &K[i] {
                    count += counts[j][k];
                }
            } else {
                for &k in &K[j] {
                    count += counts[i][k];
                }
            }
            let rational = num::rational::Ratio::new(count, all_count);
            chmax!(ans, rational);
        }
    }
    println!("{:.8}", ans.to_f64().unwrap());
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
