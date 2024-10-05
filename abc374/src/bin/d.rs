#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn distance_coordinates(a: (f64, f64), b: (f64, f64)) -> f64 {
    let h_distance = (a.0 - b.0).abs();
    let v_distance = (a.1 - b.1).abs();
    ((h_distance * h_distance) + (v_distance * v_distance)).sqrt()
}

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
fn solve() {
    input_interactive! {
        N: usize, S: f64, T: f64,
        coordinates: [(f64, f64, f64, f64); N],
    };

    let mut ans = f64::MAX;
    for bits in 0..1 << N {
        for perm in (0..N).permutations(N) {
            let mut sum = 0_f64;
            let mut pos = (0_f64, 0_f64); // (x, y)
            for i in perm {
                let (x1, y1, x2, y2) = coordinates[i];
                let pos1 = (x1, y1);
                let pos2 = (x2, y2);
                if (bits >> i) & 1 == 1 {
                    sum += distance_coordinates(pos, pos1) / S;
                    sum += distance_coordinates(pos1, pos2) / T;
                    pos = pos2;
                } else {
                    sum += distance_coordinates(pos, pos2) / S;
                    sum += distance_coordinates(pos2, pos1) / T;
                    pos = pos1;
                }
            }
            chmin!(ans, sum);
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
