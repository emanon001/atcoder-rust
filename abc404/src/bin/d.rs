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

#[allow(non_snake_case)]
fn main() {
    input_interactive! {
        N: usize, M: usize,
        C: [u64; N],
        KA: [[Usize1]; M],
    };

    let mut zoo_to_animals = vec![vec![]; N];
    for i in 0..M {
        for j in &KA[i] {
            zoo_to_animals[*j].push(i);
        }
    }

    let mut ans = std::u64::MAX;
    for x in 0_usize..3.pow(N as u32) {
        // 3進数で表す
        let mut x = x;
        let mut visit_counts = Vec::new();
        for _ in 0..N {
            visit_counts.push(x % 3);
            x /= 3;
        }

        let mut sum = 0_u64;
        let mut animal_counts = vec![0; M];
        for i in 0..N {
            sum += C[i] * visit_counts[i] as u64;
            for a in &zoo_to_animals[i] {
                animal_counts[*a] += visit_counts[i];
            }
        }
        let is_ok = animal_counts.iter().all(|&x| x >= 2);
        if is_ok {
            chmin!(ans, sum);
        }
    }
    println!("{}", ans);
}
