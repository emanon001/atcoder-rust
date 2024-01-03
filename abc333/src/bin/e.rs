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
        queries: [(usize, Usize1); N]
    };

    let mut enemy_counts = vec![0; N];
    let mut operations = VecDeque::new();
    for &(t, x) in queries.iter().rev() {
        if t == 1 {
            if enemy_counts[x] > 0 {
                operations.push_front(1);
                enemy_counts[x] -= 1;
            } else {
                operations.push_front(0);
            }
        } else {
            enemy_counts[x] += 1;
        }
    }

    if enemy_counts.into_iter().sum::<usize>() > 0 {
        println!("-1");
        return;
    }

    let mut max_potion_count = 0;
    let mut potion_count = 0;
    let mut potion_i = 0;
    for (t, _) in queries {
        if t == 1 {
            potion_count += operations[potion_i];
            potion_i += 1;
        } else {
            potion_count -= 1;
        }
        chmax!(max_potion_count, potion_count);
    }
    println!("{}", max_potion_count);
    println!("{}", operations.iter().join(" "));
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
