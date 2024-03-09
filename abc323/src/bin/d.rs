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
        N: usize,
        SC: [(u64, u64); N],
    };

    let mut ans = 0;
    let mut map = SC.into_iter().collect::<BTreeMap<_, _>>();
    loop {
        if map.is_empty() {
            break;
        }
        let (&s, &c) = map.iter().next().unwrap();
        map.remove(&s);
        if c % 2 == 1 {
            ans += 1;
        }
        let new_s = s * 2;
        let new_c = c / 2;
        if new_c > 0 {
            *map.entry(new_s).or_insert(0) += new_c;
        }
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
