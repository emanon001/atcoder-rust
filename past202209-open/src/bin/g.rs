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
        N: usize, L: usize, K: usize,
        S: [Chars; N]
    };

    let mut ans = 0;
    for bits in 0..1 << L {
        let asterisk_count = (0..L).filter(|&i| (bits >> i) & 1 == 1).count();
        if asterisk_count != K {
            continue;
        }
        let mut count_map = HashMap::new();
        for s in &S {
            let mut key = Vec::new();
            for (i, &ch) in s.iter().enumerate() {
                if (bits >> i) & 1 == 0 {
                    key.push(ch);
                }
            }
            *count_map
                .entry(key.into_iter().collect::<String>())
                .or_insert(0) += 1;
        }
        chmax!(ans, *count_map.values().max().unwrap_or(&0));
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
