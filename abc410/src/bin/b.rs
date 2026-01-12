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
fn main() {
    input_interactive! {
        N: usize, Q: usize,
        X: [usize; Q],
    };

    let mut boxes = vec![0; N + 1];
    let mut ans = vec![];
    for x in X {
        if x >= 1 {
            boxes[x] += 1;
            ans.push(x);
            continue;
        }

        let mut min_ball_box_map = BTreeMap::new();
        for i in 1..=N {
            if min_ball_box_map.contains_key(&boxes[i]) {
                continue;
            }
            min_ball_box_map.insert(boxes[i], i);
        }
        let min_ball_box = *min_ball_box_map.first_key_value().unwrap().1;
        ans.push(min_ball_box);
        boxes[min_ball_box] += 1;
    }
    println!("{}", ans.into_iter().join(" "));
}
