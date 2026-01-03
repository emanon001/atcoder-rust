#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        K: [[Usize1]; M],
        B: [Usize1; N]
    };

    // 食材を使用している料理のインデックス
    let mut food_indexies = vec![HashSet::new(); N];
    for i in 0..M {
        for &k in &K[i] {
            food_indexies[k].insert(i);
        }
    }

    let mut count = M;
    let mut ans = VecDeque::new();
    for b in B.into_iter().rev() {
        ans.push_front(count);
        let indexies = food_indexies[b].clone();
        let c = indexies.len();
        count -= c;
        for i in indexies {
            for &k in &K[i] {
                food_indexies[k].remove(&i);
            }
        }
    }
    println!("{}", ans.into_iter().join("\n"));
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
