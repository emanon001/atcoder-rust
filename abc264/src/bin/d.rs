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
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        s: String
    };

    let mut res: HashMap<String, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push((s.clone(), 0));
    res.insert(s, 0);
    while let Some((s, c)) = heap.pop() {
        if res.contains_key(&s) && res.get(&s).unwrap() < &c {
            continue;
        }
        for i in 0..6 {
            let mut chars = s.chars().collect::<Vec<_>>();
            chars.swap(i, i + 1);
            let new_s = chars.iter().join("");
            let new_c = c + 1;
            if res.contains_key(&new_s) && res.get(&new_s).unwrap() <= &new_c {
                continue;
            }
            res.insert(new_s.clone(), new_c);
            heap.push((new_s, new_c));
        }
    }
    println!("{}", res.get("atcoder").unwrap());
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
